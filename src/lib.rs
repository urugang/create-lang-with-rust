#![allow(dead_code)]
#![allow(unreachable_patterns)]
#![feature(slice_patterns)]

mod parser;
mod normalize;
mod codegen;
mod patch;
mod runtime;

use runtime::Program;
use normalize::Normalize;
use codegen::Codegen;
use patch::Patch;

fn compile(input: &str) -> Program {
    let ast = parser::parse(input);
    let ast = Normalize::default().run(ast);
    let program = Codegen::default().run(&ast);
    Patch::default().run(program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{Value, Obj, run};
    use test_case_derive::test_case;

    fn driver(input: &str) -> Option<Value> {
        dbg!(&input);
        let program = compile(input);
        run(program).result
    }

    fn driver_heap(input: &str) -> Vec<Obj> {
        let program = compile(input);
        run(program).heap
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

    #[test_case("(let [(x 5) (y 6)] in (+ x y))" => Some(Value::Number(11)) :: "local two")]
    fn local(input: &str) -> Option<Value> {
        driver(input)
    }

    #[test_case("(if false (- 10 3) (+ 2 3))" => Some(Value::Number(5)) :: "if false")]
    #[test_case("(if true (- 10 3) (+ 2 3))" => Some(Value::Number(7)) :: "if true")]
    fn conditional(input: &str) -> Option<Value> {
        driver(input)
    }

    #[test_case("(cons 5 ())" => vec![Obj::Nil, Obj::Cons(Value::Number(5), Value::Obj(0))] :: "simple cons")]
    fn lists(input: &str) -> Vec<Obj> {
        driver_heap(input)
    }

    #[test_case("(let add [x y] (+ x y) in (add 40 2))" => Some(Value::Number(42)) :: "simple func")]
    #[test_case("(let fib [n] 
        (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)) ) )
    in (fib 8))" => Some(Value::Number(21)) :: "simple recursion")]
    fn functions(input: &str) -> Option<Value> {
        driver(input)
    }
}