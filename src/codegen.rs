use std::collections::BTreeMap;
use crate::parser::SExp;
use crate::runtime::{ByteOp, Program};
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub struct Codegen {
    code: Vec<ByteOp>,
    func_code: Vec<ByteOp>,
    locals: BTreeMap<usize, usize>,
    next_local: usize,
    func: Option<FnInfo>
}

#[derive(Debug)]
struct FnInfo {
    var: usize,
    label: usize
}

lazy_static! {
    static ref ID_COUNT: AtomicUsize = AtomicUsize::new(0);
}

impl Codegen {
    pub fn run(mut self, sexp: &SExp) -> Program {
        self.visit(sexp);
        if self.func.is_none() {
            self.emit(ByteOp::Halt);
        }
        self.code.append(&mut self.func_code);
        Program {
            code: self.code
        }
    }

    pub fn with_func(mut self, var: usize, label: usize) -> Self {
        let info = FnInfo {
            var, label
        };
        self.func = Some(info);
        self.emit(ByteOp::Label(label));
        self
    }

    pub fn with_args(mut self, args: Vec<usize>) -> Self {
        for id in args {
            let local = self.next_local();
            self.locals.insert(id, local);
        }

        self
    }
    
    fn visit(&mut self, sexp: &SExp) {
        match sexp {
            SExp::ConstNumber(i) => self.emit(ByteOp::PushConstNumber(*i)),
            SExp::Ident("true") => self.emit(ByteOp::PushTrue),
            SExp::Ident("false") => self.emit(ByteOp::PushFalse),
            SExp::Var(pos) if Some(*pos) == self.func.as_ref().map(|f| f.var) => {
                let f = self.func.as_ref().unwrap();
                self.emit(ByteOp::PushFunc(f.label));
            },
            SExp::Var(pos) => {
                let local = *self.locals.get(pos).unwrap_or_else(|| panic!("Local variable at: {:?}", pos));
                self.emit(ByteOp::LoadLocal(local))
            },
            SExp::List(l) => match &l[..] {
                [SExp::Ident("+"), first, second, rest..] => {
                    self.binary_op(ByteOp::Add, first, second, rest);
                },
                [SExp::Ident("-"), first, second, rest..] => {
                    self.binary_op(ByteOp::Sub, first, second, rest);
                },
                [SExp::Ident("<"), first, second, rest..] => {
                    self.binary_op(ByteOp::Less, first, second, rest);
                },
                [SExp::Ident("let"), SExp::Var(func_ptr), SExp::List(args), fn_body, SExp::Ident("in"), body] => {
                    let label = self.next_label();
                    let mut program = Codegen::default()
                        .with_func(*func_ptr, label)
                        .with_args(args.iter().map(SExp::expect_var).collect())
                        .run(fn_body);
                    program.code.push(ByteOp::Return(1));
                    self.func_code.append(&mut program.code);

                    self.emit(ByteOp::PushFunc(label));
                    self.emit(ByteOp::AddLocal);
                    let local = self.next_local();
                    self.locals.insert(*func_ptr, local);
                    
                    self.visit(body);
                }
                [SExp::Ident("let"), SExp::List(var_decls), SExp::Ident("in"), body] => {
                    self.decl_vars(var_decls);
                    self.visit(body);
                },
                [SExp::Ident("if"), cond, then, els] => {
                    self.visit(cond);
                    let fals = self.next_label();
                    let merge = self.next_label();
                    self.emit(ByteOp::BrFalse(fals));
                    self.visit(then);
                    self.emit(ByteOp::Jump(merge));
                    self.emit(ByteOp::Label(fals));
                    self.visit(els);
                    self.emit(ByteOp::Label(merge));
                },
                [SExp::Ident("cons"), left, right] => {
                    self.visit(left);
                    self.visit(right);
                    self.emit(ByteOp::MkCons);
                },
                [callee, args..] => {
                    for arg in args  {
                        self.visit(arg);
                    }
                    self.visit(callee);
                    self.emit(ByteOp::Call(args.len()));
                },
                [] => self.emit(ByteOp::PushNil),
                _ => unreachable!("List: {:?}", l)
            }
            _ => unreachable!("SExp: {:?}", sexp)
        }
    }

    fn next_label(&mut self) -> usize {
        ID_COUNT.fetch_add(1, Ordering::SeqCst)
    }

    fn next_local(&mut self) -> usize {
        let local = self.next_local;
        self.next_local += 1;
        local
    }

    fn binary_op(&mut self, op: ByteOp, first: &SExp, second: &SExp, rest: &[SExp]) {
        self.visit(first);
        self.visit(second);
        self.emit(op);
        for exp in rest {
            self.visit(exp);
            self.emit(op);
        }
    }

    fn decl_vars(&mut self, var_decls: &[SExp]) {
        for var_decl in var_decls {
            let var_decl = var_decl.expect_list();
            let id = var_decl[0].expect_var();
            let value = var_decl[1];
            self.visit(value);
            self.emit(ByteOp::AddLocal);
            let local = self.next_local();
            self.locals.insert(id, local);
        }
    }

    fn emit(&mut self, byte: ByteOp) {
        self.code.push(byte);
    }
}
