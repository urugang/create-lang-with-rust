
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Number(i64),
    True,
    False,
    Function(usize),
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

    fn expect_func(self) -> usize {
        match self {
            Value::Function(loc)  => loc,
            els => panic!("Expect Function, found {:?}", els)
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
    MkCons,
    
    //6.
    PushFunc(usize), //label
    Call(usize), // argc
    Return(usize) // retc
}

pub struct Program {
    pub code: Vec<ByteOp>
}

pub struct ProgramResult {
    pub result: Option<Value>,
    pub heap: Vec<Obj>
}

#[derive(Default)]
struct Frame {
    return_pc: Option<usize>,
    stack:  Vec<Value>,
    locals: Vec<Value>,
}

pub fn run(program: Program) -> ProgramResult {
    use ByteOp::*;
    let code = &program.code[..];
    let mut pc = 0;
    let mut frames = vec![ Frame::default() ];
    let mut frame = frames.last_mut().unwrap();
    let mut heap:   Vec<Obj>   = vec![ Obj::Nil ];

    while let Some(op) = code.get(pc) {
        pc += 1;
        match op {
            Halt => break,
            Label(_) => unreachable!("Unreachable operation: {:?}", op),
            PushConstNumber(number) => frame.stack.push(Value::Number(*number)),
            PushTrue => frame.stack.push(Value::True),
            PushFalse => frame.stack.push(Value::False),
            PushNil => frame.stack.push(Value::Obj(0)),
            PushFunc(func) => frame.stack.push(Value::Function(*func)),
            Add => {
                let b = frame.stack.pop().expect("B in A + B").expect_number();
                let a = frame.stack.pop().expect("A in A + B").expect_number();

                frame.stack.push(Value::Number(a + b));
            },
            Sub => {
                let b = frame.stack.pop().expect("B in A - B").expect_number();
                let a = frame.stack.pop().expect("A in A - B").expect_number();

                frame.stack.push(Value::Number(a - b));
            },
            AddLocal => {
                let value = frame.stack.pop().expect("variable to store");
                frame.locals.push(value);
            },
            LoadLocal(pos) => {
                let value = frame.locals.get(*pos).expect("local variable");
                frame.stack.push(*value);
            },
            BrFalse(loc) => {
                let value = frame.stack.pop().expect("condition in if expression").expect_bool();
                
                if !value {
                    pc = *loc;
                }
            },
            Jump(loc) => {
                pc = *loc;
            },
            MkCons => {
                let right = frame.stack.pop().expect("RIGHT in cons");
                let left = frame.stack.pop().expect("LEFT in cons");
                let loc = heap.len();
                heap.push(Obj::Cons(left, right));
                frame.stack.push(Value::Obj(loc));
            },
            Call(argc) => {
                let loc = frame.stack.pop().expect("Function pointer").expect_func();
                let len = frame.stack.len() - argc;
                let args = frame.stack.split_off(len);
                let next_frame = Frame {
                    return_pc: Some(pc),
                    locals: args,
                    ..Frame::default()
                };
                frames.push(next_frame);
                frame = frames.last_mut().unwrap();
                pc = loc;
            },
            Return(retc) => {
                let len = frame.stack.len() - retc;
                let values = frame.stack.split_off(len);

                frames.pop();
                frame = frames.last_mut().unwrap();
                for val in values {
                    frame.stack.push(val);
                }
            }
        }
    }

    ProgramResult {
        result: frame.stack.pop(),
        heap:   heap
    }
}