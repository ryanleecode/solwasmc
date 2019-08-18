extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "sol.pest"]
pub struct SolParser;

use pest::error::Error;
use pest::Parser;
use std::ffi::CString;

mod primary_expr;
fn parse(source: &str) -> Result<Vec<u16>, Error<Rule>> {
  let mut ast = vec![];
  let parsed_text = SolParser::parse(Rule::program, source)?;
  println!("{:?}", parsed_text);

  Ok(ast)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let unparsed_file = std::fs::read_to_string("testfile1.sol").expect("cannot read .sol file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    assert_eq!(2 + 2, 4);
  }
}
