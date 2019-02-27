use crate::parser::SExp;
use crate::runtime::{ByteOp, Program};

#[derive(Default)]
pub struct Codegen {
    code: Vec<ByteOp>
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
                _ => unreachable!("List: {:?}", l)
            }
            _ => unreachable!("SExp: {:?}", sexp)
        }
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
