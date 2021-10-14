#![feature(wasm_abi)]

use crate2::{Error, ModuleName};

#[no_mangle]
pub extern "wasm" fn test() {
    let err = Error::InsufficientAllowance;
    format!("boom: {}", err.module_name());
}
