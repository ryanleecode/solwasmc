use crate::atom::parse_identifier;
use crate::{
  definition::{
    contract_part::{parse as parse_contract_part, ContractPart, FunctionDefinition},
    contract_type::{parse as parse_contract_type, ContractType},
  },
  elementary_type_name::{ElementaryTypeName, UInt},
  expression::{Parameter, TypeName},
  op_codes::OpCode,
  visibility::Visibility,
};
use nom::{
  character::complete::{char, multispace0, multispace1},
  combinator::{complete, map},
  dbg_dmp,
  multi::many0,
  sequence::{delimited, preceded, terminated, tuple},
  IResult,
};

mod constructor;
mod contract_part;
mod contract_type;

trait OpCodes {
  fn op_codes() -> Vec<OpCode>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Contract {
  pub contract_type: ContractType,
  pub identifier: String,
  pub contract_part: Vec<ContractPart>,
}

impl Contract {
  pub fn op_codes(self) -> Vec<u32> {
    if self.contract_type == ContractType::Contract {
      let mut codes = vec![
        OpCode::PUSH1 as u32,
        0x80,
        OpCode::PUSH1 as u32,
        0x40,
        OpCode::MSTORE as u32,
      ];

      // TODO: Maybe dynamic dispatch?
      for part in self.contract_part {
        codes.extend(part.op_codes());
      }

      return codes;
    }
    return vec![];
  }

  pub fn runtime_op_codes(self) -> Vec<u32> {
    if self.contract_type == ContractType::Contract {
      let mut codes = vec![OpCode::PUSH1 as u32, 0x40, OpCode::MLOAD as u32];

      return codes;
    }

    return vec![];
  }
}

use std::str::from_utf8;
pub fn parse_contract(i: &[u8]) -> IResult<&[u8], Contract> {
  complete(map(
    tuple((
      parse_contract_type,
      preceded(multispace1, parse_identifier),
      terminated(
        preceded(
          multispace0,
          preceded(char('{'), many0(preceded(multispace0, parse_contract_part))),
        ),
        preceded(multispace0, char('}')),
      ),
    )),
    |x| {
      let (contract_type, identifier, contract_part) = x;
      Contract {
        contract_type,
        identifier,
        contract_part: contract_part,
      }
    },
  ))(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;
  use std::str::from_utf8;
  #[test]
  fn parses_interface_contract() {
    let input = "interface GeneralERC20 { function transfer(address to, uint256 value) external; }";
    let result = parse_contract(input.as_bytes());
    if result.is_err() {
      result.expect("parses interface contract");
    } else {
      let (remaining, def) = result.ok().unwrap();
      assert_eq!(
        (from_utf8(remaining).unwrap(), def),
        (
          "",
          Contract {
            contract_type: ContractType::Interface,
            identifier: "GeneralERC20".to_string(),
            contract_part: vec![ContractPart::FunctionDefinition(FunctionDefinition {
              identifier: Some("transfer".to_string()),
              visibility: Some(Visibility::External),
              state_mutability: None,
              parameter_list: vec![
                Parameter {
                  typename: TypeName::ElementaryTypeName(ElementaryTypeName::Address),
                  storage_location: None,
                  identifier: Some("to".to_string())
                },
                Parameter {
                  typename: TypeName::ElementaryTypeName(ElementaryTypeName::UInt(UInt::Uint256)),
                  storage_location: None,
                  identifier: Some("value".to_string())
                }
              ],
              returns: vec![],
              block: vec![],
            },)]
          }
        )
      )
    }
  }

  #[test]
  fn parses_empty_contract() {
    let input = "contract ERC20 { }";
    let result = parse_contract(input.as_bytes());
    if result.is_err() {
      result.expect("parses empty contract");
    } else {
      let (remaining, def) = result.ok().unwrap();
      assert_eq!(
        (from_utf8(remaining).unwrap(), def),
        (
          "",
          Contract {
            contract_type: ContractType::Contract,
            identifier: "ERC20".to_string(),
            contract_part: vec![]
          }
        )
      )
    }
  }
}
