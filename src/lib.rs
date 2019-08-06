use wasm_bindgen::prelude::*;

#[wasm_bindgen]

pub fn compile(assem_code: &str) -> &str {
    return "";
}

#[cfg(test)]
mod tests {
    use std::fs;
    use itertools::Itertools;
    use crate::compile;

    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn generates_bytecode_for_testfile_1() {
        let contract = fs::read_to_string("testfile1.sol")
            .expect("failed to read testfile1.sol");
        let expected_bytecode = fs::read_to_string("testfile1.bin")
            .expect("failed to read testfile1.sol");
        let byte_code = compile(&contract);
        assert_eq!(byte_code, expected_bytecode)
    }
}