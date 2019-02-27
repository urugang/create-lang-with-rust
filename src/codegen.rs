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
            SExp::List(l) => match &l[..] {
                [SExp::Ident("+"), first, second, rest..] => {
                    self.binary_op(ByteOp::Add, first, second, rest);
                },
                [SExp::Ident("-"), first, second, rest..] => {
                    self.binary_op(ByteOp::Sub, first, second, rest);
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

    fn emit(&mut self, byte: ByteOp) {
        self.code.push(byte);
    }
}
