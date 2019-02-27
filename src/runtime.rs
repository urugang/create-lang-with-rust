
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Number(i64)
}

impl Value {
    fn expect_number(self) -> i64 {
        match self {
            Value::Number(i) => i,
            els => panic!("Expected Number, found {:?}", els)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ByteOp {
    //1.
    Halt,
    PushConstNumber(i64),

    //2.
    Add,
    Sub
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
            PushConstNumber(number) => stack.push(Value::Number(*number)),
            Add => {
                let b = stack.pop().expect("B in A + B").expect_number();
                let a = stack.pop().expect("A in A + B").expect_number();

                stack.push(Value::Number(a + b));
            },
            Sub => {
                let b = stack.pop().expect("B in A - B").expect_number();
                let a = stack.pop().expect("A in A - B").expect_number();

                stack.push(Value::Number(a - b));
            }
        }
    }

    stack.pop()
}