use crate::{
    definition::{parse_contract, Contract},
    directive::{parse_pragma_directive, PragmaDirective},
    op_codes::OpCode,
};
use nom::{
    character::complete::multispace0,
    combinator::map,
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Root {
    pragma_directive: PragmaDirective,
    contracts: Vec<Contract>,
}

impl Root {
    pub fn op_codes(self) -> Vec<u32> {
        let mut codes = Vec::<u32>::new();
        for contract in self.contracts {
            for code in contract.op_codes() {
                codes.push(code);
            }
        }
        codes
    }
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Root> {
    map(
        tuple((
            parse_pragma_directive,
            many0(preceded(multispace0, parse_contract)),
        )),
        |x| {
            let (pragma_directive, contracts) = x;
            Root {
                pragma_directive,
                contracts,
            }
        },
    )(i)
}
