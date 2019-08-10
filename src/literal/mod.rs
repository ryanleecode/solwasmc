use nom::{
    alt as altm,
    bytes::complete::{escaped, escaped_transform, tag, take_until},
    character::complete::{anychar, char, digit1, hex_digit1},
    combinator::{flat_map, map, not, rest},
    delimited as delimitedm, escaped_transform as escaped_transformm, is_not as is_notm,
    map as mapm, named,
    sequence::{preceded, terminated},
    tag as tagm, IResult,
};
use std::str::from_utf8;

mod boolean;
mod number_unit;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    DecimalNumber(String),
    HexNumber(String),
    StringLiteral(String),
}

// TODO: support more than integer
fn parse_decimal_number(i: &[u8]) -> IResult<&[u8], Literal> {
    map(digit1, |d| {
        Literal::DecimalNumber(from_utf8(d).ok().unwrap().to_string())
    })(i)
}

fn parse_hex_number(i: &[u8]) -> IResult<&[u8], Literal> {
    map(preceded(tag("0x"), hex_digit1), |s| {
        let mut num = from_utf8(s).ok().unwrap().to_string();
        num.insert_str(0, "0x");
        Literal::HexNumber(num)
    })(i)
}

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
    map(string, |s| Literal::StringLiteral(s))(i)
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
                    Literal::HexNumber("0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6".to_string())
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
                ("  \n", Literal::StringLiteral("hello\" world".to_string()))
            )
        }
    }

}
