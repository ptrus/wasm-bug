#![feature(wasm_abi)]

#[derive(Debug)]
pub enum Error {
    InsufficientAllowance,
}

pub trait ModuleName {
    fn module_name(&self) -> &str;
}

impl ModuleName for Error {
    fn module_name(&self) -> &str {
        "test"
    }
}
