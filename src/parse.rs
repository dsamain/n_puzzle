
use std::io::Read;
use std::process::exit;

use crate::*;

fn parse_args(h: &mut op) -> String {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if (args.len() == 0) {
        help();
        exit(0);
    }

    let mut filename = String::new();

    for e in args {
        if e.starts_with('-') {
            match e.as_str() {
                "-m" | "--manhattan" => {*h = heuristic::manhattan_distance}
                //"-h" | "--hamming" => {*h = heuristic::hamming_distance}
                "-e" | "--euclidian" => {*h = heuristic::euclidian_distance_squared}
                _ => error(format!("Unknown option \"{}\"", e.as_str()).as_str()),
            } 
        } else {
            filename = e;
        }
    }
    if (filename.len() == 0) {
        error("No file provided");
    }

    return filename;
}

pub fn parse(n: &mut u16, start: &mut Vec<Vec<u16>>, h: &mut op) {

    let filename = parse_args(h);

    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines: Vec<&str> = contents.split(|c| c == '\n').collect();

    let mut i: usize = 0;

    let mut line;
    while i < lines.len() {
        line = String::new();
        let line = lines[i].chars().take_while(|c| *c != '#').collect::<String>();         
        let line = line.split_whitespace().collect::<Vec<&str>>();
        i += 1;
        //dbg!(&line);
        match line.len() {
            0 => {
                if i == lines.len() {
                    error("Maybe you forgot to put a map in the file?");
                }
                continue;
            }
            1 => {
                *n = line[0].parse::<u16>().unwrap();
                break;
            }
            _ => {error("first line must contain only one number");}
        }
    }

    while i < lines.len() {
        let line = lines[i];
        let line = line.chars().take_while(|&c| c != '#' && c != '\n').collect::<String>();
        let line = line.split_whitespace().collect::<Vec<&str>>();

        i += 1;

        if line.len() == 0 {
            continue;
        }

        if line.len() != 0 && start.len() == *n as usize {
            error("Invalid map");
        }

        if (line.len() != *n as usize) {
            error(format!("line {} must contain {} numbers", i, n).as_str());
        }


        start.push(line.iter().map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>());
    }

    if start.len() != *n as usize {
        error(format!("map of dimension {} must contain {} numbers", n, *n * *n).as_str());
    }
}