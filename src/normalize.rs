
use std::collections::BTreeMap;
use crate::parser::SExp;

#[derive(Default)]
struct Scope<'a> {
    locals: BTreeMap<&'a str, usize>
}

#[derive(Default)]
pub struct Normalize<'a> {
    scopes: Vec<Scope<'a>>,
    next_id: Vec<usize>,
}

impl<'a> Normalize<'a> {
    pub fn run(mut self, sexp: SExp<'a>) -> SExp<'a> {
        self.next_id.push(0);
        self.scopes.push(Scope::default());
        self.visit(&sexp)
    }

    fn visit(&mut self, sexp: &SExp<'a>) -> SExp<'a> {
        match sexp {
            SExp::List(l) => match &l[..] {
                [_let @ SExp::Ident("let"), SExp::List(var_decls), _in @ SExp::Ident("in"), body] => {
                    self.scopes.push(Scope::default());
                    let mut normalized = vec![];
                    for var_decl in var_decls.iter().map(SExp::expect_list) {
                        let name = var_decl[0].expect_ident();
                        let id = self.next_id();
                        self.scope().locals.insert(name, id);
                        let value = self.visit(var_decl[1]);
                        normalized.push(SExp::List(vec![ SExp::Var(id), value ]));
                    }
                    let body = self.visit(body);
                    self.scopes.pop();
                    SExp::List(vec![_let.clone(), SExp::List(normalized), _in.clone(), body])
                },
                [SExp::Ident("let"), SExp::Ident(name), SExp::List(args), fn_body, SExp::Ident("in"), body] => {
                    self.scopes.push(Scope::default());
                    let id = self.next_id();
                    self.scope().locals.insert(name, id);
                    let name = SExp::Var(id);
                    self.scopes.push(Scope::default());
                    let mut normalized = vec![];
                    for arg in args {
                        let id = self.next_id();
                        self.scope().locals.insert(arg.expect_ident(), id);
                        normalized.push(SExp::Var(id));
                    }
                    let fn_body = self.visit(fn_body);
                    self.scopes.pop();
                    let body = self.visit(body);
                    self.scopes.pop();
                    SExp::List(vec![SExp::Ident("let"), name, SExp::List(normalized), fn_body, SExp::Ident("in"), body])
                }
                _ => {
                    let list = l.iter().map(|sexp| self.visit(sexp)).collect();
                    SExp::List(list)
                }
            },
            SExp::Ident(name) => {
                match self.get_pos(name) {
                    Some(pos) => SExp::Var(pos),
                    None => sexp.clone()
                }
            }
            _ => sexp.clone()
        }
    }

    fn get_pos(&self, name: &'a str) -> Option<usize> {
        for scope in self.scopes.iter().rev() {
            if let Some(pos) = scope.locals.get(name) {
                return Some(*pos);
            }
        }
        None
    }

    fn next_id(&mut self) -> usize  {
        let id = *self.next_id.last().unwrap();
        let next_id = self.next_id.last_mut().unwrap();
        *next_id += 1;
        id
    }

    fn scope(&mut self) -> &mut Scope<'a> {
        self.scopes.last_mut().expect("Scope")
    }
}