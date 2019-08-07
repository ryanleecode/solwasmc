use wasm_bindgen::prelude::*;

mod atom;
#[allow(dead_code)]
mod delimiter;
mod expression;
#[allow(dead_code)]
mod keyword;
#[allow(dead_code)]
mod reserved;
mod statement;
mod token;

pub fn compile(assem_code: &str) -> &str {
  /*    let mut lexer = Token::lexer(assem_code);

  while lexer.token != Token::End {
      if lexer.token == Token::Error {
          println!("ERROR: {}", lexer.slice());
          break;
      }
      println!("{}", lexer.slice());
      lexer.advance();
  }*/
  return "";
}

#[cfg(test)]
mod tests {
  use crate::compile;
  use std::fs;

  use pretty_assertions::assert_eq;

  #[test]
  fn generates_bytecode_for_testfile_1() {
    let contract = fs::read_to_string("testfile1.sol").expect("failed to read testfile1.sol");
    let expected_bytecode =
      fs::read_to_string("testfile1.bin").expect("failed to read testfile1.sol");
    let byte_code = compile(&contract);
    assert_eq!(byte_code, expected_bytecode)
  }
}
