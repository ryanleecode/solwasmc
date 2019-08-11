use crate::literal::boolean::parse as parse_bool;
use crate::literal::number_unit::{parse as parse_number_unit, NumberUnit};
use nom::{
    alt as altm,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, multispace1},
    combinator::{map, opt},
    delimited as delimitedm, escaped_transform as escaped_transformm, is_not as is_notm,
    map as mapm, named,
    sequence::{preceded, tuple},
    tag as tagm, IResult,
};
use std::str::from_utf8;

mod boolean;
mod number_unit;

pub use boolean::Boolean;

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Hex(String),
    Decimal(String),
}

pub type NumberLiteral = (Number, Option<NumberUnit>);

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(NumberLiteral),
    String(String),
    Boolean(Boolean),
}

// TODO: support more than integer
fn parse_decimal_number(i: &[u8]) -> IResult<&[u8], Number> {
    map(digit1, |d| {
        Number::Decimal(from_utf8(d).ok().unwrap().to_string())
    })(i)
}

fn parse_hex_number(i: &[u8]) -> IResult<&[u8], Number> {
    map(preceded(tag("0x"), hex_digit1), |s| {
        let mut num = from_utf8(s).ok().unwrap().to_string();
        num.insert_str(0, "0x");
        Number::Hex(num)
    })(i)
}

// TODO: Parse Hex Literal
fn parse_hex_literal() {}

fn parse_string_literal(i: &[u8]) -> IResult<&[u8], Literal> {
    fn to_s(i: Vec<u8>) -> String {
        String::from_utf8_lossy(&i).into_owned()
    }

    named!(
        string_content<String>,
        mapm!(
            escaped_transformm!(
                is_notm!("\"\\"),
                '\\',
                altm!(
                    tagm!("\\") => { |_| &b"\\"[..] } |
                    tagm!("\"") => { |_| &b"\""[..] } |
                    tagm!("n") => { |_| &b"\n"[..] } |
                    tagm!("r") => { |_| &b"\r"[..] } |
                    tagm!("t") => { |_| &b"\t"[..] }
                )
            ),
            to_s
        )
    );
    named!(
        string<String>,
        delimitedm!(tagm!("\""), string_content, tagm!("\""))
    );
    map(string, |s| Literal::String(s))(i)
}

fn parse_number(i: &[u8]) -> IResult<&[u8], Number> {
    alt((parse_hex_number, parse_decimal_number))(i)
}

fn parse_number_literal(i: &[u8]) -> IResult<&[u8], NumberLiteral> {
    tuple((parse_number, opt(preceded(multispace1, parse_number_unit))))(i)
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Literal> {
    alt((
        parse_string_literal,
        map(parse_bool, |b| Literal::Boolean(b)),
        map(parse_number_literal, |n| Literal::Number(n)),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
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
                ("\n", Number::Decimal("323".to_string()))
            )
        }
    }

    #[test]
    fn parses_hex_number() {
        let input = b"0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6\n";
        let result = parse_hex_number(input);
        if result.is_err() {
            result.expect("should parse break");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), b),
                (
                    "\n",
                    Number::Hex("0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6".to_string())
                )
            )
        }
    }

    #[test]
    fn parses_string_literal() {
        let input = b"\"hello\\\" world\"  \n";
        let result = parse_string_literal(input);
        if result.is_err() {
            result.expect("should parse string literal");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), b),
                ("  \n", Literal::String("hello\" world".to_string()))
            )
        }
    }

    #[test]
    fn hex_number_takes_precedence_over_decimal() {
        let input = b"0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6\n";
        let result = parse_number(input);
        if result.is_err() {
            result.expect("should parse break");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), b),
                (
                    "\n",
                    Number::Hex("0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6".to_string())
                )
            )
        }
    }

}
