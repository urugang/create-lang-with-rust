
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Number(i64),
    True,
    False
}

impl Value {
    fn expect_number(self) -> i64 {
        match self {
            Value::Number(i) => i,
            els => panic!("Expected Number, found {:?}", els)
        }
    }
    
    fn expect_bool(self) -> bool {
        match self {
            Value::True  => true,
            Value::False => false,
            els => panic!("Expect Boolean, found {:?}", els)
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
    Sub,

    //3.
    AddLocal,
    LoadLocal(usize),

    //4.
    PushTrue,
    PushFalse,

    Label(usize), //only for patching. Unreachable in runtime
    BrFalse(usize),
    Jump(usize)
}

pub struct Program {
    pub code: Vec<ByteOp>
}

pub fn run(program: Program) -> Option<Value> {
    use ByteOp::*;
    let code = &program.code[..];
    let mut pc = 0;
    let mut stack:  Vec<Value> = vec![];
    let mut locals: Vec<Value> = vec![];

    while let Some(op) = code.get(pc) {
        pc += 1;
        match op {
            Halt => break,
            Label(_) => unreachable!("Unreachable operation: {:?}", op),
            PushConstNumber(number) => stack.push(Value::Number(*number)),
            PushTrue => stack.push(Value::True),
            PushFalse => stack.push(Value::False),
            Add => {
                let b = stack.pop().expect("B in A + B").expect_number();
                let a = stack.pop().expect("A in A + B").expect_number();

                stack.push(Value::Number(a + b));
            },
            Sub => {
                let b = stack.pop().expect("B in A - B").expect_number();
                let a = stack.pop().expect("A in A - B").expect_number();

                stack.push(Value::Number(a - b));
            },
            AddLocal => {
                let value = stack.pop().expect("variable to store");
                locals.push(value);
            },
            LoadLocal(pos) => {
                let value = locals.get(*pos).expect("local variable");
                stack.push(*value);
            },
            BrFalse(loc) => {
                let value = stack.pop().expect("condition in if expression").expect_bool();
                
                if !value {
                    pc = *loc;
                }
            },
            Jump(loc) => {
                pc = *loc;
            }
        }
    }

    stack.pop()
}