
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Number(i64)
}

#[derive(Debug, PartialEq)]
pub enum ByteOp {
    Halt,
    PushConstNumber(i64)
}

pub struct Program {
    pub code: Vec<ByteOp>
}

pub fn run(program: Program) -> Option<Value> {
    use ByteOp::*;
    let code = &program.code[..];
    let mut pc = 0;
    let mut stack: Vec<Value> = vec![];

    while let Some(op) = code.get(pc) {
        pc += 1;
        match op {
            Halt => break,
            PushConstNumber(number) => stack.push(Value::Number(*number))
        }
    }

    stack.pop()
}