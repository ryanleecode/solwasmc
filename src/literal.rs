use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

const TRUE: &str = "true";
const FALSE: &str = "false";

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BooleanLiteral {
    True,
    False,
}

fn parse_true(i: &[u8]) -> IResult<&[u8], BooleanLiteral> {
    map(tag(TRUE), |_| BooleanLiteral::True)(i)
}

fn parse_false(i: &[u8]) -> IResult<&[u8], BooleanLiteral> {
    map(tag(FALSE), |_| BooleanLiteral::False)(i)
}

fn parse_boolean(i: &[u8]) -> IResult<&[u8], BooleanLiteral> {
    alt((parse_true, parse_false))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::from_utf8;

    #[test]
    fn parses_true() {
        let input = b"true\n";
        let result = parse_true(input);
        if result.is_err() {
            result.expect("should parse true");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), b),
                ("\n", BooleanLiteral::True)
            )
        }
    }

    #[test]
    fn parses_continue() {
        let input = b"false\n";
        let result = parse_false(input);
        if result.is_err() {
            result.expect("should parse false");
        } else {
            let (remaining, c) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), c),
                ("\n", BooleanLiteral::False)
            )
        }
    }
}
