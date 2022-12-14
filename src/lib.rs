pub type Op = fn(&Vec<Vec<u16>>, &Vec<(u16, u16)>, u16) -> u32;

pub mod heuristic;
pub mod init;
pub mod parse;

pub use rustc_hash::FxHashMap;
pub use std::collections::BTreeMap;
pub use std::collections::BinaryHeap;
pub use std::env;
pub use std::fs;
pub use std::rc::Rc;

pub use heuristic::*;
pub use init::*;
pub use parse::*;

pub enum Mode {
    Astar = 0,
    Greedy = 1,
    Uniformcost = 2,
}

pub const FLAG_O: u32 = 1 << 0;
pub const FLAG_R: u32 = 1 << 1;

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
    std::process::exit(1);
}

pub fn help() {
    println!("Usage: ./npuzzle [options] file\n");

    println!("Options:");
    println!("    --help      | -h  : Show this help message");
    println!("    --reverse   | -r  : print solution in reverse order");
    println!("    --order     | -o  : solution become tiles in increasing order");
    println!("");
    println!("Variante:");
    println!("    --astar     | -a  : use A* (default)");
    println!("    --greedy    | -g  : use greedy search (only heuristic)");
    println!("    --uniform   | -u  : use uniformCost search (bfs)");
    println!("");
    println!("Heuristic:");
    println!("    --euclidian | -e  : use euclidian distance (default)");
    println!("    --manhattan | -m  : use manhattan distance");
    println!("    --linear    | -l  : use linear conflict");
    println!("    --misplaced | -t  : use misplaced tiles");
}
