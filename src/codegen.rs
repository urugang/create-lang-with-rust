use crate::parser::SExp;
use crate::runtime::{ByteOp, Program};

#[derive(Default)]
pub struct Codegen {
    code: Vec<ByteOp>,
    next_label: usize
}

impl Codegen {
    pub fn run(mut self, sexp: &SExp) -> Program {
        self.visit(sexp);
        Program {
            code: self.code
        }
    }

    fn visit(&mut self, sexp: &SExp) {
        match sexp {
            SExp::ConstNumber(i) => self.emit(ByteOp::PushConstNumber(*i)),
            SExp::Ident("true") => self.emit(ByteOp::PushTrue),
            SExp::Ident("false") => self.emit(ByteOp::PushFalse),
            SExp::Var(pos) => self.emit(ByteOp::LoadLocal(*pos)),
            SExp::List(l) => match &l[..] {
                [SExp::Ident("+"), first, second, rest..] => {
                    self.binary_op(ByteOp::Add, first, second, rest);
                },
                [SExp::Ident("-"), first, second, rest..] => {
                    self.binary_op(ByteOp::Sub, first, second, rest);
                },
                [SExp::Ident("let"), SExp::List(var_decls), SExp::Ident("in"), body] => {
                    dbg!(&body);
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
                _ => unreachable!("List: {:?}", l)
            }
            _ => unreachable!("SExp: {:?}", sexp)
        }
    }

    fn next_label(&mut self) -> usize {
        let label = self.next_label;
        self.next_label += 1;
        label
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
            let _id = var_decl[0].expect_var();
            let value = var_decl[1];
            self.visit(value);
            self.emit(ByteOp::AddLocal);
        }
    }

    fn emit(&mut self, byte: ByteOp) {
        self.code.push(byte);
    }
}
