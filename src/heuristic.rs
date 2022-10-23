
pub fn misplaced_tiles(state: &Vec<u16>, target_map: &Vec<u16>) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..state.len() {
        ret += (target_map[state[i] as usize] != i as u16) as u32;
    }
    return ret as u32;
}

pub fn manhattan_distance(state: &Vec<u16>, target_map: &Vec<u16>) -> u32 {
    let n = (state.len() as f64).sqrt() as u32;
    let mut ret: u32 = 0;
    for i in 0..state.len() {
        let diff: u32 = (target_map[state[i] as usize] as i32 - i as i32).abs() as u32;
        ret += (diff / n + diff % n);
    }
    return ret as u32;
}

pub fn euclidian_distance_squared(state: &Vec<u16>, target_map: &Vec<u16>) -> u32 {
    let n = (state.len() as f64).sqrt() as u32;
    let mut ret: u32 = 0;
    for i in 0..state.len() {
        let diff: u32 = (target_map[state[i] as usize] as i32 - i as i32).abs() as u32;
        ret += ((diff / n) * (diff / n)) + (diff % n) * (diff % n);
    }
    return ret as u32;
}