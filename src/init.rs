
pub fn set_target(n: u16, target_state: &mut Vec<u16>, target_map: &mut Vec<u16>) {
    let mut cur: u16 = 1;
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut k: usize = 0;

    *target_state = vec![0; (n * n) as usize];
    *target_map = vec![0; (n * n) as usize];

    let dir_x: &[i16] = &[1, 0, -1, 0];
    let dir_y: &[i16] = &[0, 1, 0, -1];

    while cur < n * n {
        target_state[(i * n as i16 + j) as usize] = cur;
        if i + dir_y[k % 4 ] < 0 || i + dir_y[k % 4] >= n as i16 
            || j + dir_x[k % 4 ] < 0 || j + dir_x[k % 4] >= n as i16
            || target_state[((i + dir_y[k % 4]) * n as i16 + j + dir_x[k % 4]) as usize] != 0 {
            k += 1;
        }
        cur += 1;
        i += dir_y[k % 4];
        j += dir_x[k % 4];
    }

    for i in 0..n {
        for j in 0..n {
            target_map[target_state[(i * n + j) as usize] as usize] = i * n + j;
        }
    }

    println!("target_state: {:?}", target_state);
    println!("target_map: {:?}", target_map);
}
