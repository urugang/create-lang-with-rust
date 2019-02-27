
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Number(i64),
    True,
    False,
    Obj(usize)
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

    fn expect_ref(self) -> usize {
        match self {
            Value::Obj(loc)  => loc,
            els => panic!("Expect Reference, found {:?}", els)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Obj {
    Nil,
    Cons(Value, Value)
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
    Jump(usize),

    //5.
    PushNil,
    MkCons
}

pub struct Program {
    pub code: Vec<ByteOp>
}

pub struct ProgramResult {
    pub result: Option<Value>,
    pub heap: Vec<Obj>
}

pub fn run(program: Program) -> ProgramResult {
    use ByteOp::*;
    let code = &program.code[..];
    let mut pc = 0;
    let mut stack:  Vec<Value> = vec![];
    let mut locals: Vec<Value> = vec![];
    let mut heap:   Vec<Obj>   = vec![ Obj::Nil ];

    while let Some(op) = code.get(pc) {
        pc += 1;
        match op {
            Halt => break,
            Label(_) => unreachable!("Unreachable operation: {:?}", op),
            PushConstNumber(number) => stack.push(Value::Number(*number)),
            PushTrue => stack.push(Value::True),
            PushFalse => stack.push(Value::False),
            PushNil => stack.push(Value::Obj(0)),
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
            },
            MkCons => {
                let right = stack.pop().expect("RIGHT in cons");
                let left = stack.pop().expect("LEFT in cons");
                let loc = heap.len();
                heap.push(Obj::Cons(left, right));
                stack.push(Value::Obj(loc));
            }
        }
    }

    ProgramResult {
        result: stack.pop(),
        heap:   heap
    }
}