
use std::io::Read;

use crate::*;

pub fn parse(n: &mut u16, start: &mut Vec<Vec<u16>>, h: &mut op) {

    //parse_args(&mut h);

    let contents = std::fs::read_to_string("map/map.txt").expect("Something went wrong reading the file");
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
                    panic!("Maybe you forgot to put a map in the file?");
                }
                continue;
            }
            1 => {
                *n = line[0].parse::<u16>().unwrap();
                break;
            }
            _ => {panic!("first line must contain only one number");}
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

        if (line.len() != *n as usize) {
            panic!("line {} must contain {} numbers", i, n);
        }

        start.push(line.iter().map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>());

        //for j in 0..*n {
            //let num = line[j as usize].parse::<u16>().unwrap();
            //if num < 0 || num > *n * *n - 1 {
                //panic!("number {} is out of range", num);
            //}
            //if start.contains(&num) {
                //panic!("number {} is duplicated", num);
            //}
            //start.last().push(num);
        //}
    }

    if start.len() != *n as usize {
        panic!("map must contain {} numbers", *n * *n);
    }
}