use crate::atom::parse_identifier;
use nom::{
    character::complete::{char, multispace0},
    multi::separated_nonempty_list,
    sequence::preceded,
    IResult,
};

fn parse_assembly_identifier_list(i: &[u8]) -> IResult<&[u8], Vec<String>> {
    separated_nonempty_list(char(','), preceded(multispace0, parse_identifier))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::from_utf8;

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
