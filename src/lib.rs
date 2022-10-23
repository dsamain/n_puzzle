
pub type Binop = fn(&Vec<u16>, &Vec<u16>) -> u32;

pub mod heuristic;
pub mod parse;
pub mod init;

pub use std::collections::BTreeMap;
pub use std::env;
pub use std::fs;

pub use heuristic::*;
pub use parse::*;
pub use init::*;
