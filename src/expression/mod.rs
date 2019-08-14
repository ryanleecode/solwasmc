use crate::atom::parse_identifier;
use crate::elementary_type_name::{parse as parse_elementary_type_name, ElementaryTypeName, UInt};
use crate::expression::{
  assignment::{parse as parse_assignment_operator, Assignment},
  function::parses_function_call,
  primary_expr::parse as parse_primary_expression,
};
use crate::literal::{Boolean, Literal, Number};
use crate::op_codes::OpCode;
use crate::storage_location::{parse as parse_storage_location, StorageLocation};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_till1, take_until},
  character::complete::{char, multispace0, multispace1},
  combinator::{complete, flat_map, map, map_res, peek},
  multi::many1,
  multi::{separated_list, separated_nonempty_list},
  sequence::{delimited, preceded, separated_pair, terminated, tuple},
  IResult,
};
use sha3::{Digest, Keccak256};
use std::collections::HashSet;
use std::u32;
mod assignment;
mod function;
mod primary_expr;

pub use crate::expression::{function::FunctionCallArguments, primary_expr::PrimaryExpression};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
  pub expr: Box<Expression>,
  pub args: FunctionCallArguments,
}

impl FunctionCall {
  fn hex_to_bytes(addr: &str) -> Vec<u32> {
    let mut bytes = vec![];

    let mut index = 0;
    while index < addr.len() {
      let byte_str = format!(
        "{}{}",
        addr.chars().nth(index).unwrap(),
        addr.chars().nth(index + 1).unwrap()
      );
      let byte = u32::from_str_radix(&byte_str.as_str(), 16).unwrap();
      bytes.push(byte);
      index += 2;
    }

    bytes
  }

  pub fn op_codes(self) -> Vec<u32> {
    let mut codes = vec![];

    // TODO: Bring the interfaces as a parameters and check if the
    // identifier is one of them
    let interfaces: HashSet<&'static str> = ["GeneralERC20"].iter().cloned().collect();
    let interface_contract_addr = self.interface_contract_address(interfaces);
    if interface_contract_addr != None {
      codes.push(OpCode::PUSH20 as u32);
      let addr = interface_contract_addr.unwrap();
      let addr_bytes = FunctionCall::hex_to_bytes(&addr);
      codes.extend(addr_bytes);
      // MASK
      codes.push(OpCode::PUSH20 as u32);
      for _ in 0..20 {
        codes.push(0xff);
      }
      codes.push(OpCode::AND as u32);

      return codes;
    }

    let sig = self.signature();
    let mut hasher = Keccak256::new();
    hasher.input(sig);
    let hash = &hasher.result()[..];
    codes.push(OpCode::PUSH4 as u32);
    for byte in hash[0..4].iter() {
      codes.push(*byte as u32);
    }
    codes.extend(self.param_bytecode());

    codes
  }

  fn param_bytecode(&self) -> Vec<u32> {
    let mut bytecode = vec![];

    match self.expr.as_ref() {
      Expression::PrimaryExpression(state) => match state {
        PrimaryExpression::Identifier(identifier) => match &self.args {
          FunctionCallArguments::ExpressionList(list) => {
            for expr in list {
              match expr {
                Expression::PrimaryExpression(prim_expr) => {
                  match prim_expr {
                    PrimaryExpression::NumberLiteral(arg_identifier) => {
                      let (number, _) = arg_identifier;
                      match number {
                        Number::Hex(val) => {
                          bytecode.push(OpCode::PUSH20 as u32);
                          let bytes = FunctionCall::hex_to_bytes(val.trim_start_matches("0x"));
                          bytecode.extend(bytes);
                        }
                        Number::Decimal(val) => {
                          bytecode.push(OpCode::PUSH1 as u32);
                          let hex_str = format!("{:x}", val.parse::<i32>().unwrap());
                          let bytes = FunctionCall::hex_to_bytes(hex_str.trim_start_matches("0x"));
                          bytecode.extend(bytes);
                          /*    let bytes = FunctionCall::hex_to_bytes(val.trim_start_matches("0x"));
                          bytecode.push(OpCode::PUSH1 as u32);
                          bytecode.extend(bytes); */
                          // TODO: FIX THE HARDCODED TYPES
                          // args.push("uint256".to_string());
                        }
                      }
                    }
                    PrimaryExpression::BooleanLiteral(bool_lit) => match bool_lit {
                      Boolean::False => {
                        bytecode.push(0x00);
                      }
                      Boolean::True => {
                        bytecode.push(0x01);
                      }
                    },
                    _ => {
                      panic!("{:#?} is not a valid parameter", prim_expr);
                    }
                  }
                }
                _ => {}
              }
            }
          }
          _ => panic!(
            "{:#?} function call arguments should be an expression list",
            self.args
          ),
        },
        _ => panic!("{:#?} should be the identifier of the function call", state),
      },
      _ => panic!(
        "{:#?} should be a primary expression of the function call",
        self.expr
      ),
    }

    bytecode
  }

  fn interface_contract_address(&self, interfaces: HashSet<&'static str>) -> Option<String> {
    match self.expr.as_ref() {
      Expression::PrimaryExpression(state) => match state {
        PrimaryExpression::Identifier(identifier) => {
          if (interfaces.get(&identifier.as_str())) == None {
            return None;
          }
          match &self.args {
            FunctionCallArguments::ExpressionList(list) => {
              let mut args = Vec::<String>::new();
              for expr in list {
                match expr {
                  Expression::PrimaryExpression(prim_expr) => {
                    match prim_expr {
                      PrimaryExpression::NumberLiteral(arg_identifier) => {
                        let (number, _) = arg_identifier;
                        match number {
                          Number::Hex(val) => {
                            // TODO: FIX THE HARDCODED TYPES
                            args.push(val.to_string());
                          }
                          _ => {
                            panic!("{:#?} is not a valid address", number);
                          }
                        }
                      }
                      _ => {
                        panic!("{:#?} is not a valid address", prim_expr);
                      }
                    }
                  }
                  _ => {}
                }
              }

              if args.len() != 1 {
                return None;
              }
              let addr = &args[0].trim_start_matches("0x");
              return Some(addr.to_string());
            }
            _ => panic!(
              "{:#?} function call arguments should be an expression list",
              self.args
            ),
          }
        }
        _ => panic!("{:#?} should be the identifier of the function call", state),
      },
      _ => panic!(
        "{:#?} should be a primary expression of the function call",
        self.expr
      ),
    }
  }
  fn signature(&self) -> String {
    match self.expr.as_ref() {
      Expression::PrimaryExpression(state) => match state {
        PrimaryExpression::Identifier(identifier) => match &self.args {
          FunctionCallArguments::ExpressionList(list) => {
            let mut args = Vec::<String>::new();
            for expr in list {
              match expr {
                Expression::PrimaryExpression(prim_expr) => {
                  match prim_expr {
                    PrimaryExpression::NumberLiteral(arg_identifier) => {
                      let (number, _) = arg_identifier;
                      match number {
                        Number::Hex(val) => {
                          // TODO: FIX THE HARDCODED TYPES
                          args.push("address".to_string());
                        }
                        Number::Decimal(val) => {
                          // TODO: FIX THE HARDCODED TYPES
                          args.push("uint256".to_string());
                        }
                      }
                    }
                    PrimaryExpression::BooleanLiteral(bool_lit) => {
                      let mut lit = "true";
                      match bool_lit {
                        Boolean::False => {
                          lit = "false";
                        }
                        _ => {}
                      }
                      args.push(format!("bool {}", lit));
                    }
                    _ => {
                      panic!("{:#?} is not a valid parameter", prim_expr);
                    }
                  }
                }
                _ => {}
              }
            }

            format!("{}({})", identifier, args.join(","))
          }
          _ => panic!(
            "{:#?} function call arguments should be an expression list",
            self.args
          ),
        },
        _ => panic!("{:#?} should be the identifier of the function call", state),
      },
      _ => panic!(
        "{:#?} should be a primary expression of the function call",
        self.expr
      ),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  // TODO: PostFix(),
  // TODO: New(),
  // TODO: IndexAccess,
  MemberAccess(Box<Expression>, Box<Expression>),
  FunctionCall(FunctionCall),
  // TODO:   ('!' | '~' | 'delete' | '++' | '--' | '+' | '-') Expression
  // TODO: | Expression '**' Expression
  // TODO: | Expression ('*' | '/' | '%') Expression
  // TODO: | Expression ('+' | '-') Expression
  // TODO: | Expression ('<<' | '>>') Expression
  // TODO: | Expression '&' Expression
  // TODO: | Expression '^' Expression
  // TODO: | Expression '|' Expression
  // TODO: | Expression ('<' | '>' | '<=' | '>=') Expression
  // TODO: | Expression ('==' | '!=') Expression
  // TODO: | Expression '&&' Expression
  // TODO: | Expression '||' Expression
  // TODO: | Expression '?' Expression ':' Expression
  Assignment(Box<Expression>, Assignment, Box<Expression>),
  // TODO: | Expression ('=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '+=' | '-=' | '*=' | '/=' | '%=') Expression
  PrimaryExpression(PrimaryExpression),
}

impl Expression {
  pub fn op_codes(self) -> Vec<u32> {
    let mut codes = vec![];
    match self {
      Expression::MemberAccess(expr1, expr2) => {
        // TODO: SOME CUSTOM LOGIC

        codes.extend(expr1.op_codes());
        codes.extend(expr2.op_codes());
      }
      Expression::FunctionCall(fn_call) => codes.extend(fn_call.op_codes()),
      _ => {}
    }
    codes
  }
}

pub fn parse_expression(i: &[u8]) -> IResult<&[u8], Expression> {
  alt((
    map(parse_member_access, |m| {
      let (exp, mem) = m;
      Expression::MemberAccess(Box::new(exp), Box::new(mem))
    }),
    map(parses_function_call, |f| {
      let (expr, args) = f;
      Expression::FunctionCall(FunctionCall {
        expr: Box::new(expr),
        args,
      })
    }),
    delimited(char('('), parse_expression, char(')')),
    map(parse_assignment_expression, |x| {
      let (expr1, op, expr2) = x;
      Expression::Assignment(expr1, op, expr2)
    }),
    map(parse_primary_expression, |e| {
      Expression::PrimaryExpression(e)
    }),
  ))(i)
}

pub fn parse_expr_without_assignment(i: &[u8]) -> IResult<&[u8], Expression> {
  alt((
    map(parse_member_access, |m| {
      let (exp, mem) = m;
      Expression::MemberAccess(Box::new(exp), Box::new(mem))
    }),
    map(parses_function_call, |f| {
      let (expr, args) = f;
      Expression::FunctionCall(FunctionCall {
        expr: Box::new(expr),
        args,
      })
    }),
    delimited(char('('), parse_expression, char(')')),
    map(parse_primary_expression, |e| {
      Expression::PrimaryExpression(e)
    }),
  ))(i)
}

fn parse_assignment_expression(
  i: &[u8],
) -> IResult<&[u8], (Box<Expression>, Assignment, Box<Expression>)> {
  complete(tuple((
    map(preceded(multispace0, parse_expr_without_assignment), |x| {
      Box::new(x)
    }),
    preceded(multispace0, parse_assignment_operator),
    map(preceded(multispace0, parse_expr_without_assignment), |x| {
      Box::new(x)
    }),
  )))(i)
}

pub fn parse_expression_list(i: &[u8]) -> IResult<&[u8], Vec<Expression>> {
  separated_nonempty_list(char(','), preceded(multispace0, parse_expression))(i)
}

fn parse_member_access(i: &[u8]) -> IResult<&[u8], (Expression, Expression)> {
  map_res(
    separated_pair(
      take_until("."),
      char('.'),
      alt((
        map(parses_function_call, |x| {
          let (expr, args) = x;
          Expression::FunctionCall(FunctionCall {
            expr: Box::new(expr),
            args,
          })
        }),
        map(parse_identifier, |x| {
          Expression::PrimaryExpression(PrimaryExpression::Identifier(x))
        }),
      )),
    ),
    |x| {
      let (expr, id) = x;
      let p_expr = parse_expression(expr);
      if p_expr.is_err() {
        return Err(p_expr.unwrap_err());
      } else {
        let (_, e) = p_expr.unwrap();
        return Ok((e, id));
      }
    },
  )(i)
}

pub type ElementaryTypeNameExpression = ElementaryTypeName;

#[derive(Debug, PartialEq, Clone)]
pub enum TypeName {
  ElementaryTypeName(ElementaryTypeName),
  UserDefinedTypeName(Vec<String>),
  // TODO: Mapping
  // TODO: ArrayTypeName
  // TODO: FunctionTypeName
  // TODO: ( 'address' 'payable' )
}

pub fn parse_user_defined_type_name(i: &[u8]) -> IResult<&[u8], TypeName> {
  map(separated_nonempty_list(char('.'), parse_identifier), |x| {
    TypeName::UserDefinedTypeName(x)
  })(i)
}

pub fn parse_type_name(i: &[u8]) -> IResult<&[u8], TypeName> {
  alt((
    map(parse_elementary_type_name, |e| {
      TypeName::ElementaryTypeName(e)
    }),
    parse_user_defined_type_name,
  ))(i)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
  pub typename: TypeName,
  pub storage_location: Option<StorageLocation>,
  pub identifier: Option<String>,
}

use std::str::from_utf8;
pub fn parse_parameter(i: &[u8]) -> IResult<&[u8], Parameter> {
  map(
    tuple((
      parse_type_name,
      alt((
        complete(map(
          tuple((
            preceded(multispace1, parse_storage_location),
            preceded(multispace1, parse_identifier),
          )),
          |tup| {
            let (storage, id) = tup;
            (Some(storage), Some(id.to_string()))
          },
        )),
        complete(map(preceded(multispace1, parse_identifier), |id| {
          (None, Some(id.to_string()))
        })),
        map(multispace0, move |_| (None, None)),
      )),
    )),
    |t| {
      let (typename, params) = t;
      let (storage_location, identifier) = params;
      Parameter {
        typename,
        storage_location,
        identifier,
      }
    },
  )(i)
}

pub fn parse_parameter_list(i: &[u8]) -> IResult<&[u8], Vec<Parameter>> {
  terminated(
    preceded(
      char('('),
      preceded(
        multispace0,
        separated_list(
          preceded(multispace0, char(',')),
          preceded(multispace0, parse_parameter),
        ),
      ),
    ),
    preceded(multispace0, char(')')),
  )(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::elementary_type_name::ElementaryTypeName;
  use crate::expression::function::FunctionCallArguments;
  use crate::literal::Number;
  use pretty_assertions::assert_eq;
  use std::str::from_utf8;

  #[test]
  fn parses_user_defined_type_name() {
    let input = "OpenZepp.ERC20.ABC {";
    let (remaining, typename) = parse_user_defined_type_name(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), typename),
      (
        " {",
        TypeName::UserDefinedTypeName(vec![
          "OpenZepp".to_string(),
          "ERC20".to_string(),
          "ABC".to_string()
        ])
      )
    )
  }

  #[test]
  fn parses_user_defined_type_name_with_no_periods() {
    let input = "keccak";
    let (remaining, typename) = parse_user_defined_type_name(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), typename),
      (
        "",
        TypeName::UserDefinedTypeName(vec!["keccak".to_string(),])
      )
    )
  }

  #[test]
  fn parses_fully_qualified_parameter() {
    let input = "bool     memory     isWorking\n";
    let (remaining, param) = parse_parameter(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), param),
      (
        "\n",
        Parameter {
          typename: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
          storage_location: Some(StorageLocation::Memory),
          identifier: Some("isWorking".to_string()),
        }
      )
    )
  }

  #[test]
  fn parses_parameter_with_identifier_only() {
    let input = "bool    isWorking\n";
    let (remaining, param) = parse_parameter(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), param),
      (
        "\n",
        Parameter {
          typename: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
          storage_location: None,
          identifier: Some("isWorking".to_string()),
        }
      )
    )
  }

  #[test]
  fn parse_parameter_tolerate_uint256() {
    let input = "uint256 value";
    let (remaining, param) = parse_parameter(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), param),
      (
        "",
        Parameter {
          typename: TypeName::ElementaryTypeName(ElementaryTypeName::UInt(UInt::Uint256)),
          storage_location: None,
          identifier: Some("value".to_string()),
        }
      )
    )
  }

  #[test]
  fn parses_parameter_with_type_only() {
    let input = "bool   \n";
    let result = parse_parameter(input.as_bytes());
    if result.is_err() {
      result.expect("error");
    } else {
      let (remaining, param) = result.ok().unwrap();
      assert_eq!(
        (from_utf8(remaining).unwrap(), param),
        (
          "",
          Parameter {
            typename: TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
            storage_location: None,
            identifier: None,
          }
        )
      )
    }
  }

  #[test]
  fn parses_parameter_list_no_params() {
    let input = "(    )";
    let result = parse_parameter_list(input.as_bytes());
    if result.is_err() {
      result.expect("error");
    } else {
      let (remaining, params) = result.ok().unwrap();
      assert_eq!((from_utf8(remaining).unwrap(), params), ("", Vec::new()))
    }
  }

  #[test]
  fn parses_parameter_list_one_param() {
    let input = "(   address   )";
    let result = parse_parameter_list(input.as_bytes());
    if result.is_err() {
      result.expect("error");
    } else {
      let (remaining, params) = result.ok().unwrap();
      assert_eq!(
        (from_utf8(remaining).unwrap(), params),
        (
          "",
          vec![Parameter {
            typename: TypeName::ElementaryTypeName(ElementaryTypeName::Address),
            storage_location: None,
            identifier: None
          }]
        )
      )
    }
  }

  #[test]
  fn parses_parameter_list_mutliple_params() {
    let input = "(address   to   ,   uint   age)";
    let result = parse_parameter_list(input.as_bytes());
    if result.is_err() {
      result.expect("error");
    } else {
      let (remaining, params) = result.ok().unwrap();
      assert_eq!(
        (from_utf8(remaining).unwrap(), params),
        (
          "",
          vec![
            Parameter {
              typename: TypeName::ElementaryTypeName(ElementaryTypeName::Address),
              storage_location: None,
              identifier: Some("to".to_string()),
            },
            Parameter {
              typename: TypeName::ElementaryTypeName(ElementaryTypeName::UInt(UInt::Uint)),
              storage_location: None,
              identifier: Some("age".to_string()),
            }
          ]
        )
      )
    }
  }

  #[test]
  fn parses_member_access() {
    let input = "aaaa.bbbb\n";
    let result = parse_member_access(input.as_bytes());
    if result.is_err() {
      result.expect("error");
    } else {
      let (remaining, params) = result.ok().unwrap();
      assert_eq!(
        (from_utf8(remaining).unwrap(), params),
        (
          "\n",
          (
            Expression::PrimaryExpression(PrimaryExpression::Identifier("aaaa".to_string())),
            Expression::PrimaryExpression(PrimaryExpression::Identifier("bbbb".to_string()))
          )
        )
      )
    }
  }

  #[test]
  fn parses_member_access2() {
    let input = "aa.b\n";
    let result = parse_member_access(input.as_bytes());
    if result.is_err() {
      result.expect("error");
    } else {
      let (remaining, params) = result.ok().unwrap();
      assert_eq!(
        (from_utf8(remaining).unwrap(), params),
        (
          "\n",
          (
            Expression::PrimaryExpression(PrimaryExpression::Identifier("aa".to_string())),
            Expression::PrimaryExpression(PrimaryExpression::Identifier("b".to_string()))
          )
        )
      )
    }
  }

  #[test]
  fn parses_member_access3() {
    let input = "a.bb\n";
    let result = parse_member_access(input.as_bytes());
    if result.is_err() {
      result.expect("error");
    } else {
      let (remaining, params) = result.ok().unwrap();
      assert_eq!(
        (from_utf8(remaining).unwrap(), params),
        (
          "\n",
          (
            Expression::PrimaryExpression(PrimaryExpression::Identifier("a".to_string())),
            Expression::PrimaryExpression(PrimaryExpression::Identifier("bb".to_string()))
          )
        )
      )
    }
  }

  #[test]
  fn parses_fnc_call_with_member_access_fnc_call() {
    let input = "GeneralERC20(0xf25186B5081Ff5cE73482AD761DB0eB0d25abfBF).transfer(0x821aEa9a577a9b44299B9c15c88cf3087F3b5544, 250)\n";
    let result = parse_member_access(input.as_bytes());
    if result.is_err() {
      result.expect("error");
    } else {
      let (remaining, params) = result.ok().unwrap();
      assert_eq!(
        (from_utf8(remaining).unwrap(), params),
        (
          "\n",
          (
            Expression::FunctionCall(FunctionCall {
              expr: Box::new(Expression::PrimaryExpression(
                PrimaryExpression::Identifier("GeneralERC20".to_string())
              )),
              args: FunctionCallArguments::ExpressionList(vec![Expression::PrimaryExpression(
                PrimaryExpression::NumberLiteral((
                  Number::Hex("0xf25186B5081Ff5cE73482AD761DB0eB0d25abfBF".to_string()),
                  None
                ))
              )])
            }),
            Expression::FunctionCall(FunctionCall {
              expr: Box::new(Expression::PrimaryExpression(
                PrimaryExpression::Identifier("transfer".to_string())
              )),
              args: FunctionCallArguments::ExpressionList(vec![
                Expression::PrimaryExpression(PrimaryExpression::NumberLiteral((
                  Number::Hex("0x821aEa9a577a9b44299B9c15c88cf3087F3b5544".to_string()),
                  None
                ))),
                Expression::PrimaryExpression(PrimaryExpression::NumberLiteral((
                  Number::Decimal("250".to_string()),
                  None
                )))
              ])
            }),
          )
        )
      )
    }
  }
}
