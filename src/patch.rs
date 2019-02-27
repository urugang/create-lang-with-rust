use std::collections::BTreeMap;
use crate::runtime::{Program, ByteOp};

#[derive(Default)]
pub struct Patch {
    labels: BTreeMap<usize, usize>
}

impl Patch {
    pub fn run(mut self, mut program: Program) -> Program {
        self.fill_table(&program);
        self.patch(&mut program);
        program
    }
    
    fn fill_table(&mut self, program: &Program) {
        let mut pos = 0;
        for op in program.code.iter() {
            match op {
                ByteOp::Label(label) => {
                    self.labels.insert(*label, pos);
                },
                _ => pos += 1
            }
        }
    }

    fn patch(&mut self, program: &mut Program) {
        program.code = program.code.iter()
            .filter_map(|op| match op {
                ByteOp::Label(_) => None,
                ByteOp::Jump(label) => Some(ByteOp::Jump(self.labels[label])),
                ByteOp::BrFalse(label) => Some(ByteOp::BrFalse(self.labels[label])),
                op => Some(*op)
            }).collect();
    }
}