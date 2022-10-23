
pub type Binop = fn(&Vec<u16>, &Vec<u16>) -> u32;

pub mod parse;

pub use std::collections::BTreeMap;
pub use parse::*;

pub use std::env;
pub use std::fs;