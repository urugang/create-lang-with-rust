use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SExpParser;

#[derive(Debug, PartialEq, Clone)]
pub enum SExp<'a> {
    ConstNumber(i64),
    Ident(&'a str),
    Var(usize),
    List(Vec<SExp<'a>>)
}

impl<'a> SExp<'a> {
    pub fn expect_list(&self) -> Vec<&SExp<'a>> {
        match self {
            SExp::List(l) => l.iter().collect(),
            _ => panic!("Expected List found {:?}", self)
        }
    }

    pub fn expect_ident(&self) -> &'a str {
        match self {
            SExp::Ident(i) => i,
            _ => panic!("Expected Ident found {:?}", self)
        }
    }

    pub fn expect_var(&self) -> usize {
        match self {
            SExp::Var(i) => *i,
            _ => panic!("Expected Var found {:?}", self)
        }
    }
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
        Rule::ident => {
            SExp::Ident(pair.as_str())
        },
        Rule::list => {
            let list = pair.into_inner().map(parse_sexp).collect();
            SExp::List(list)
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