

use std::{cmp::Reverse, collections::HashMap};
use n_puzzle::*;

//use std::rc::Rc;
use std::cmp::{Ordering, max};
//use crate::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Puzzle {
    pub state: Rc<Vec<Vec<u16>>>,
    pub par: Option<Rc<Puzzle>>,
    pub idx: (usize, usize),
    pub cost: i32,
    pub fcost: i32,
    pub n: u16,
}

impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Puzzle) -> Option<Ordering> {
        if self.fcost == other.fcost {
            return Some(other.cost.cmp(&self.cost));
        }
        Some(other.fcost.cmp(&self.fcost))
    }
}

impl Ord for Puzzle {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.fcost == other.fcost {
            return other.cost.cmp(&self.cost);
        }
        other.fcost.cmp(&self.fcost)
    }
}


struct Stats {
    pub moves: u32,
    pub max_open: u32,
    pub total_open: u32,
}


fn reconstruct_path(puzzle: &Puzzle) -> Vec<Rc<Puzzle>> {
    let mut ret = Vec::new();
    let mut p = puzzle;
    while let Some(par) = &p.par {
        ret.push(Rc::clone(par));
        p = par;
    }
    ret
}

fn print_path(path: &Vec<Rc<Puzzle>>, stats: &Stats) {
    for p in path {
        for i in 0..p.n {
            for j in 0..p.n {
                print!("{:2} ", p.state[i as usize][j as usize]);
            }
            println!();
        }
        println!();
    }
    println!("Max simultaneous evaluted state : {}", stats.max_open);
    println!("Number of state evaluated :       {}", stats.total_open);
    println!("Numbers of moves :                {}", path.len() - 1);

}

fn get_zero(state: &Vec<Vec<u16>>) -> (usize, usize) {
    for i in 0..state.len() {
        for j in 0..state.len() {
            if state[i][j] == 0 {
                return (i, j);
            }
        }
    }
    panic!("No zero found");
}

fn a_star(mut start: Rc<Puzzle>, target_state: &Vec<Vec<u16>>, target_map: &Vec<(u16, u16)>, h: op, n: u16, stats: &mut Stats) {

    let mut open_set: BinaryHeap<Rc<Puzzle>> = BinaryHeap::new();
    let mut closed_set: FxHashMap<Rc<Vec<Vec<u16>>>, Rc<Puzzle>> = FxHashMap::default(); 

    let dx: &[i32] = &[0, 0, -1, 1];
    let dy: &[i32] = &[-1, 1, 0, 0];

    closed_set.insert(start.state.clone(), start.clone());
    open_set.push(start);

    'main_loop: while let Some(cur) = open_set.pop() {
        if cur.cost > (*closed_set.get(&cur.state).unwrap()).cost {
            continue;
        }

        if cur.fcost == cur.cost {
            break ;
        }

        let (i, j) = cur.idx;

        for k in 0..4 {
            let (mut y, mut x) = (i as i32 + dy[k], j as i32 + dx[k]);
            if x < 0 || x as u16 >= n || y < 0 || y as u16 >= n {
                continue;
            }
            let (x, y) = (x as usize, y as usize);

            let new_cost  = cur.cost + 1;

            let mut new_state = (*cur.state).clone();
            new_state[i][j] = new_state[y][x];
            new_state[y][x] = 0;

            let new_state = Rc::new(new_state);

            if closed_set.contains_key(&new_state) {
               if  (*closed_set.get(&new_state).unwrap()).cost <= new_cost {
                continue;
               } 
            } else {
                stats.total_open += 1;
            } 

            let fcost =  new_cost + (h(&new_state, &target_map, n) as i32) as i32;
            let new_puzzle = Rc::new(Puzzle{ state: new_state, 
                                             par: Some(Rc::clone(&cur)), 
                                             cost: new_cost, 
                                             fcost: fcost, 
                                             idx: (y, x), 
                                             n: n });

            closed_set.insert(new_puzzle.state.clone(), new_puzzle.clone());
            if new_puzzle.cost == new_puzzle.fcost {
                break 'main_loop;
            }
            open_set.push(new_puzzle);

        }
        stats.max_open = max(stats.max_open, open_set.len() as u32);
    }
    let path = reconstruct_path(closed_set.get(target_state).unwrap());
    print_path(&path, stats);
}

fn main() {
    let mut target_state: Vec<Vec<u16>> = Vec::new();
    let mut target_map: Vec<(u16,u16)> = Vec::new();
    let mut start_state: Vec<Vec<u16>> = Vec::new();
    let mut n: u16 = 0;


    //let mut h: op = nothing;
    //let mut h: op = misplaced_tiles;
    //let mut h: op = manhattan_distance;
    let mut h: op = euclidian_distance_squared;
    parse(&mut n, &mut start_state, &mut h);
    set_target(n, &mut target_state, &mut target_map);

    println!("n = {}", n);
    println!("target_state = {:?}", target_state);

    let mut idx: (usize, usize) = get_zero(&start_state);

    let mut stats = &mut Stats{moves: 0, max_open: 1, total_open: 1};
    let fcost = h(&start_state, &target_map, n) as i32;
    let start = Rc::new(Puzzle{ 
            state: Rc::new(start_state), 
            par: None, 
            cost: 0, 
            fcost: fcost,
            idx: idx, 
            n: n
    });

    a_star(start, &target_state, &target_map, h, n, stats);

}
