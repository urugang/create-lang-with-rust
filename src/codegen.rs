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
            _ => unreachable!("SExp: {:?}", sexp)
        }
    }

    fn emit(&mut self, byte: ByteOp) {
        self.code.push(byte);
    }
}
