extern crate pest;

#[derive(Parser)]
#[grammar = "sol.pest"]
pub struct SolParser;

use pest::error::Error;
use pest::Parser;
use pest::{consumes_to, parses_to};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_member_access() {
        let input = "a.b";
        let result = SolParser::parse(Rule::expression, input);
        println!("{:#?}", result);
        parses_to! {
            parser: SolParser,
            input: input,
            rule: Rule::expression,
            tokens: [
                member_access(0, 3, [
                    primary_expression(0, 1, [
                        identifier(0,1)
                    ]),
                    identifier(2,3),
                ])
            ]
        };
    }

    #[test]
    fn parses_index_access() {
        let input = "a[0]";
        let result = SolParser::parse(Rule::expression, input);
        println!("{:#?}", result);
        parses_to! {
            parser: SolParser,
            input: input,
            rule: Rule::expression,
            tokens: [
                index_access(0, 4, [
                    primary_expression(0, 1, [
                        identifier(0, 1, [])
                    ]),
                    primary_expression(2, 3, [
                        number_literal(2, 3, [
                            decimal_number(2, 3, [])
                        ])
                    ])
                ]),

            ]
        };
    }
}
