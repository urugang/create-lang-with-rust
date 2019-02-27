use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SExpParser;

#[derive(Debug, PartialEq)]
pub enum SExp {
    ConstNumber(i64)
}

pub fn parse(input: &str) -> SExp {
    let pair = SExpParser::parse(Rule::sexp, input)
        .unwrap().next().unwrap(); //Very nice error handling ;)
    parse_sexp(pair)
}

fn parse_sexp(pair: Pair<Rule>) -> SExp {
    match pair.as_rule() {
        Rule::number => {
            SExp::ConstNumber(str::parse(pair.as_str()).unwrap())
        },
        rule => unreachable!("RULE: {:?}", rule)
    }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn parse_number() {
       let input = "5";
       let result = parse(input);

       assert_eq!(result, SExp::ConstNumber(5));
   }
}