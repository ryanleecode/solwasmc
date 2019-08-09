use crate::atom::parse_identifier;
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::map,
    multi::separated_nonempty_list,
    sequence::preceded,
    branch::{alt},
    IResult,
};

fn parse_assembly_identifier_list(i: &[u8]) -> IResult<&[u8], Vec<String>> {
    separated_nonempty_list(char(','), preceded(multispace0, parse_identifier))(i)
}

const BREAK: &str = "break";
const CONTINUE: &str = "continue";

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyBreakContinue {
    Break,
    Continue,
}

fn parse_break(i: &[u8]) -> IResult<&[u8], AssemblyBreakContinue> {
    map(tag(BREAK), |_| AssemblyBreakContinue::Break)(i)
}

fn parse_continue(i: &[u8]) -> IResult<&[u8], AssemblyBreakContinue> {
    map(tag(CONTINUE), |_| AssemblyBreakContinue::Continue)(i)
}

fn parse_break_continue(i: &[u8]) -> IResult<&[u8], AssemblyBreakContinue> {
    alt((parse_break, parse_continue))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::from_utf8;

    #[test]
    fn parses_break() {
        let input = b"break\n";
        let result = parse_break(input);
        if result.is_err() {
            result.expect("should parse break");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), b),
                ("\n", AssemblyBreakContinue::Break)
            )
        }
    }

    #[test]
    fn parses_continue() {
        let input = b"continue\n";
        let result = parse_continue(input);
        if result.is_err() {
            result.expect("should parse continue");
        } else {
            let (remaining, c) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), c),
                ("\n", AssemblyBreakContinue::Continue)
            )
        }
    }

    #[test]
    fn parses_assem_identifier_list() {
        let input = "a,    b,      result\n";
        let result = parse_assembly_identifier_list(input.as_bytes());
        if result.is_err() {
            result.expect("error");
        } else {
            let (remaining, identifiers) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), identifiers),
                (
                    "\n",
                    vec!["a".to_string(), "b".to_string(), "result".to_string()]
                )
            )
        }
    }
}
