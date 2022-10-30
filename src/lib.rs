
pub type op = fn(&Vec<Vec<u16>>, &Vec<(u16, u16)>, u16) -> u32;

pub mod heuristic;
pub mod parse;
pub mod init;

pub mod puzzle;

pub use std::collections::BTreeMap;
pub use std::collections::BinaryHeap;
pub use std::env;
pub use std::fs;
pub use std::rc::Rc;
pub use rustc_hash::FxHashMap;

pub use puzzle::*;
pub use heuristic::*;
pub use parse::*;
pub use init::*;

pub enum Mode {
    Astar = 0,
    Greedy = 1,
    Uniformcost = 2,
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
    std::process::exit(1);
}

pub fn help() {
    println!("Usage: ./npuzzle [options] file\n");
    
    println!("Options:\n");

    println!("Variante:");
    println!("    --astar     | -a  : use A* (default)");
    println!("    --greedy    | -g  : use greedy search (only heuristic)");
    println!("    --uniform   | -u  : use uniformCost search (bfs)");
    println!("");
    println!("Heuristic:");
    println!("    --euclidian | -e  : use euclidian distance (default)");
    println!("    --hamming   | -h  : use hamming distance");
    println!("    --manhattan | -m  : use manhattan distance");
    println!("    --linear    | -l  : use linear conflict");
}