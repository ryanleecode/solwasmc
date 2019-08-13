use crate::{
    definition::{parse_contract, Contract},
    directive::{parse_pragma_directive, PragmaDirective},
    op_codes::OpCode,
};
use nom::{
    character::complete::multispace0,
    multi::many0,
     combinator::{map},
    sequence::{preceded, tuple},
    IResult,
};

pub struct Root {
    pragma_directive: PragmaDirective,
    contracts: Vec<Contract>,
}

impl Root {
    pub fn op_codes(self) -> Vec<u32> {
        vec![OpCode::PUSH1 as u32, 0x80, OpCode::PUSH1 as u32, 0x40]
    }
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Root> {
        map(tuple((
            parse_pragma_directive,
            many0(preceded(multispace0, parse_contract)),
        )),
 |x| {
        let (pragma_directive, contracts) = x;
        Root{pragma_directive, contracts}
    })(i)
} 
