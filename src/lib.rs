use crate::root::parse;
use console_error_panic_hook;
use wasm_bindgen::prelude::*;

mod atom;
mod definition;
mod directive;
mod elementary_type_name;
mod expression;
mod literal;
mod root;
#[allow(dead_code)]
mod state_mutability;
mod statement;
#[allow(dead_code)]
mod storage_location;
mod token;
#[allow(dead_code)]
mod visibility;

mod op_codes;

#[wasm_bindgen]
pub fn compile(assem_code: &str) -> String {
    console_error_panic_hook::set_once();
    let compile_result = parse(assem_code.as_bytes());
    if compile_result.is_err() {
        // let err = compile_result.unwrap_err();
        // TODO: Figure out how to get anything from this error
        panic!("compile failed");
    } else {
        let (_, root) = compile_result.ok().unwrap();

        let mut bytecode = String::new();
        for code in root.op_codes() {
            let hex = format!("{:x}", code);
            bytecode.push_str(&hex);
        }
        return bytecode;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
