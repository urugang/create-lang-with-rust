#![allow(dead_code)]
#![allow(unreachable_patterns)]
#![feature(slice_patterns)]

mod parser;
mod codegen;
mod runtime;

use runtime::Program;
use codegen::Codegen;

fn compile(input: &str) -> Program {
    let ast = parser::parse(input);
    let program = Codegen::default().run(&ast);
    program
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{Value, run};
    use test_case_derive::test_case;

    fn driver(input: &str) -> Option<Value> {
        let program = compile(input);
        run(program)
    }

    #[test_case("5"  => Some(Value::Number(5))  :: "positive number")]
    #[test_case("-5" => Some(Value::Number(-5)) :: "negative number")]
    fn number(input: &str) -> Option<Value> {
        driver(input)
    }

    #[test_case("(+ 5 6)" => Some(Value::Number(11)) :: "add")]
    #[test_case("(- 10 6)" => Some(Value::Number(4)) :: "sub")]
    #[test_case("(+ (+ 3 2) 6)" => Some(Value::Number(11)) :: "nested add")]
    #[test_case("(+ 5 6 7)" => Some(Value::Number(18)) :: "variadic operator")]
    #[test_case("(- 10 2 1)" => Some(Value::Number(7)) :: "variadic operator precedence")]
    fn binary(input: &str) -> Option<Value> {
        driver(input)
    }
}