
use n_puzzle::*;

// generate the snake solution 
fn set_target_state(n: u16, tar: &mut Vec<u16>) {
    let mut cur: u16 = 1;
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut k: usize = 0;

    *tar = vec![0; (n * n) as usize];

    let dir_x: &[i16] = &[1, 0, -1, 0];
    let dir_y: &[i16] = &[0, 1, 0, -1];

    while cur < n * n {
        tar[(i * n as i16 + j) as usize] = cur;
        if i + dir_y[k % 4 ] < 0 || i + dir_y[k % 4] >= n as i16 
            || j + dir_x[k % 4 ] < 0 || j + dir_x[k % 4] >= n as i16
            || tar[((i + dir_y[k % 4]) * n as i16 + j + dir_x[k % 4]) as usize] != 0 {
            k += 1;
        }
        cur += 1;
        i += dir_y[k % 4];
        j += dir_x[k % 4];
    }
}

fn main() {
    let mut closed_set: BTreeMap<Vec<u16>, (u32, Vec<u16>)> = BTreeMap::new(); // .0 = cost, .1 = par
    let mut open_set: BTreeMap<Vec<u16>, (u32, Vec<u16>)> = BTreeMap::new();
    let mut tar: Vec<u16> = Vec::new();
    let mut start: Vec<u16> = Vec::new();
    let mut n: u16 = 0;

    type Binop = fn(&Vec<u16>, &Vec<u16>) -> u32;

    //let mut g: Binop = nothing;
    let mut h: Binop = nothing;

    parse(&mut n, &mut start, &mut h);

    println!("n = {}", n);
    println!("start = {:?}", start);

    set_target_state(n, &mut tar);
    println!("tar = {:?}", tar);
}
