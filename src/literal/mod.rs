use nom::{character::complete::digit1, combinator::map, multi::many1, sequence::tuple, IResult};
use std::str::from_utf8;

mod boolean;
mod number_unit;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    DecimalNumber(String),
}

// TODO: support more than integer
fn parse_decimal_number(i: &[u8]) -> IResult<&[u8], Literal> {
    map(digit1, |d| {
        Literal::DecimalNumber(from_utf8(d).ok().unwrap().to_string())
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_normal_decimal_number() {
        let input = b"323\n";
        let result = parse_decimal_number(input);
        if result.is_err() {
            result.expect("should parse break");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), b),
                ("\n", Literal::DecimalNumber("323".to_string()))
            )
        }
    }
}
