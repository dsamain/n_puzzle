
use std::cmp::{Ordering, max};
use n_puzzle::*;

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
    pub max_open: u32,
    pub total_open: u32,
}

fn reconstruct_path(puzzle: &Puzzle) -> Vec<Rc<Puzzle>> {
    let mut ret = vec![Rc::new(puzzle.clone())];
    let mut p = puzzle;
    while let Some(par) = &p.par {
        ret.push(Rc::clone(par));
        p = par;
    }
    ret
}

fn print_path(path: &Vec<Rc<Puzzle>>, stats: &Stats, flag: u32) {
    if (flag & FLAG_R) == 0 {
        for p in path.iter().rev() {
            println!("╔{}╗", "═".repeat((p.n * 3) as usize));
            for i in 0..p.n {
                print!("║");
                for j in 0..p.n {
                    let c = p.state[i as usize][j as usize];
                    if c != 0 {
                        print!("{:2} ", c);
                    } else {
                        print!("   ");
                    }
                }
                println!("║");
            }
            println!("╚{}╝", "═".repeat((p.n * 3) as usize));
            println!();
        }
    } else {
        for p in path.iter() {
            println!("╔{}╗", "═".repeat((p.n * 3) as usize));
            for i in 0..p.n {
                print!("║");
                for j in 0..p.n {
                    let c = p.state[i as usize][j as usize];
                    if c != 0 {
                        print!("{:2} ", c);
                    } else {
                        print!("   ");
                    }
                }
                println!("║");
            }
            println!("╚{}╝", "═".repeat((p.n * 3) as usize));
            println!();
        }
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

fn solve(start: Rc<Puzzle>, target_state: &Vec<Vec<u16>>, target_map: &Vec<(u16, u16)>, h: Op, n: u16, stats: &mut Stats, mode: &Mode, flag: u32) {

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

        if &(*cur.state) == target_state {
            break ;
        }

        let (i, j) = cur.idx;

        for k in 0..4 {
            let (y, x) = (i as i32 + dy[k], j as i32 + dx[k]);
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

            let fcost = match mode {
                &Mode::Astar => {new_cost + (h(&new_state, &target_map, n) as i32) as i32}
                &Mode::Greedy => {(h(&new_state, &target_map, n) as i32) as i32}
                &Mode::Uniformcost => {new_cost}
            };

            let new_puzzle = Rc::new(Puzzle{ state: new_state, 
                                             par: Some(Rc::clone(&cur)), 
                                             cost: new_cost, 
                                             fcost: fcost, 
                                             idx: (y, x), 
                                             n: n });

            closed_set.insert(new_puzzle.state.clone(), new_puzzle.clone());
            //if new_puzzle.cost == new_puzzle.fcost {
            if &(*new_puzzle.state) == target_state {
                break 'main_loop;
            }
            open_set.push(new_puzzle);

        }
        stats.max_open = max(stats.max_open, open_set.len() as u32);
    }
    println!("solved\n");
    let path = reconstruct_path(closed_set.get(target_state).unwrap());
    print_path(&path, stats, flag);
}

// count the number of inversions in the puzzle, if odd -> unsolvable
fn check_solvable(state: &Vec<Vec<u16>>, target_map: &Vec<(u16, u16)>, n: u16) -> bool {
    let mut cnt = 0;
    let mut v: Vec<u16> = Vec::new();

    let mut zero_row: u16 = 0;

    for i in 0..n as usize {
        for j in 0..n as usize {
            if state[i][j] == 0 {
                zero_row = i as u16;
                continue;
            }
            let cur = target_map[state[i][j] as usize];
            v.push(cur.0 * n + cur.1);
        }
    }

    for i in 0..(v.len() - 1) {
        for j in (i+1)..v.len() {
            cnt += (v[j] < v[i]) as u16;
        }
    }
    //return true;
    return if n % 2 == 1 {
        cnt % 2 == 0
    } else {
        cnt % 2 == zero_row % 2
    };
}

fn main() {
    let mut target_state: Vec<Vec<u16>> = Vec::new();
    let mut target_map: Vec<(u16,u16)> = Vec::new();
    let mut start_state: Vec<Vec<u16>> = Vec::new();
    let mut n: u16 = 0;


    let mut h: Op = euclidian_distance_squared;
    let mut mode: Mode = Mode::Astar;
    let mut flag: u32 = 0;
    parse(&mut n, &mut start_state, &mut h, &mut mode, &mut flag);
    set_target(n, &mut target_state, &mut target_map, flag);

    //dbg!(&target_map);
    //dbg!(&target_state);

    if !check_solvable(&start_state, &target_map, n) {
        println!("Puzzle is unsolvable");
        return ;
    }


    let idx: (usize, usize) = get_zero(&start_state);

    let stats = &mut Stats{max_open: 1, total_open: 1};
    let fcost = match mode {
        Mode::Astar => {h(&start_state, &target_map, n) as i32}
        Mode::Greedy => {h(&start_state, &target_map, n) as i32}
        Mode::Uniformcost => {0}
    };
    let start = Rc::new(Puzzle{ 
            state: Rc::new(start_state), 
            par: None, 
            cost: 0, 
            fcost: fcost,
            idx: idx, 
            n: n
    });

    solve(start, &target_state, &target_map, h, n, stats, &mode, flag);

}
