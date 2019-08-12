use crate::{
    definition::{parse_contract, Contract},
    directive::{parse_pragma_directive, PragmaDirective},
};
use nom::{
    character::complete::multispace0,
    dbg_dmp,
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};

pub fn parse(i: &[u8]) -> IResult<&[u8], (PragmaDirective, Vec<Contract>)> {
    dbg_dmp(
        tuple((
            parse_pragma_directive,
            many0(preceded(multispace0, parse_contract)),
        )),
        "root parse err",
    )(i)
}
