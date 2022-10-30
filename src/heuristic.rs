
use std::collections::BinaryHeap;

pub fn nothing(state: &Vec<u16>, target_map: &Vec<u16>, n: i32) -> u32 {
    0
}

//pub fn misplaced_tiles(state: &Vec<u16>, target_map: &Vec<u16>) -> u32 {
    //let mut ret: u32 = 0;
    //for i in 0..state.len() {
        //ret += (target_map[state[i] as usize] != i as u16) as u32;
    //}
    //return ret as u32;
//}

//pub fn manhattan_distance(state: &Vec<u16>, target_map: &Vec<u16>, n: i32) -> u32 {
    //let n = n as u32;
    //let mut ret: u32 = 0;
    //for i in 0..state.len() {
        //if state[i] == 0 {
            //continue;
        //}
        //let diff: u32 = (target_map[state[i] as usize] as i32 - i as i32).abs() as u32;
        ////println!("cost for {} is {}", state[i], diff / n + diff % n);
        //ret += (diff / n + diff % n);
    //}
    //return ret as u32;
//}
#[inline]
pub fn manhattan_distance(state: &Vec<Vec<u16>>, target_map: &Vec<(u16, u16)>, n: u16) -> u32 {
    let n = n as i32;
    let mut ret: i32 = 0;
    for i in 0..state.len() {
        for j in 0..state.len() {
            if state[i][j] == 0 {
                continue;
            }
            ret += ((target_map[state[i][j] as usize].0 as i32 - i as i32).abs() + (target_map[state[i][j] as usize].1 as i32 - j as i32).abs()) as i32;
        }
    }
    return ret as u32;
}

#[inline]
pub fn euclidian_distance_squared(state: &Vec<Vec<u16>>, target_map: &Vec<(u16, u16)>, n: u16) -> u32 {
    let n = n as i32;
    let mut ret: i32 = 0;
    for i in 0..state.len() {
        for j in 0..state.len() {
            if state[i][j] == 0 {
                continue;
            }
            ret += ((target_map[state[i][j] as usize].0 as i32 - i as i32).pow(2) + (target_map[state[i][j] as usize].1 as i32 - j as i32).pow(2)) as i32;
        }
    }
    return ret as u32;
}

#[inline]
pub fn linear_conflict(state: &Vec<Vec<u16>>, target_map: &Vec<(u16, u16)>, n: u16) -> u32 {
    let n = n as i32;
    let mut ret: i32 = 0;
    for i in 0..state.len() {
        {
            let mut pq: BinaryHeap<u16> = BinaryHeap::new();
            for j in 0..state.len() {
                if target_map[state[i][j] as usize].0 != i as u16 || state[i][j] == 0 {
                    continue;
                }
                if !pq.is_empty() && pq.peek().unwrap() > &target_map[state[i][j] as usize].1 {
                    ret += 2;
                }
                pq.push(target_map[state[i][j] as usize].1);
            }
        }
        {
            let mut pq: BinaryHeap<u16> = BinaryHeap::new();
            for j in 0..state.len() {
                if target_map[state[j][i] as usize].1 != j as u16 || state[i][j] == 0 {
                    continue;
                }
                if !pq.is_empty() && pq.peek().unwrap() > &target_map[state[j][i] as usize].0 {
                    ret += 2;
                }
                pq.push(target_map[state[j][i] as usize].0);
            }
        }

    }
    return ret as u32 + manhattan_distance(state, target_map, n as u16);
}