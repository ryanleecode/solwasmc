use nom::{IResult, multi::{separated_nonempty_list}};
use crate::atom::{parse_identifier};

fn parse_assembly_identifier_list(i: &[u8]) -> IResult<&[u8], Vec<String>> {
    separated_nonempty_list(char(','), parse_identifier)(i)
}
