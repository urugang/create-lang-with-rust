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
}