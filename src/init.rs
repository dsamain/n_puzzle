use crate::*;

pub fn set_target(n: u16, target_state: &mut Vec<Vec<u16>>, target_map: &mut Vec<(u16, u16)>, flag: u32) {

    let mut cur: u16 = 1;
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut k: usize = 0;

    *target_state = vec![vec![0; n as usize]; n as usize];
    *target_map = vec![(0, 0); (n * n) as usize];

    if (flag & FLAG_O) == 0 {
        let dir_x: &[i16] = &[1, 0, -1, 0];
        let dir_y: &[i16] = &[0, 1, 0, -1];

        while cur < n * n {
            target_state[i as usize][j as usize] = cur;
            if i + dir_y[k % 4] < 0
                || i + dir_y[k % 4] >= n as i16
                || j + dir_x[k % 4] < 0
                || j + dir_x[k % 4] >= n as i16
                || target_state[(i + dir_y[k % 4]) as usize][(j + dir_x[k % 4]) as usize] != 0
            {
                k += 1;
            }
            cur += 1;
            i += dir_y[k % 4];
            j += dir_x[k % 4];
        }
    } else {
        for i in 0..n as usize {
            for j in 0..n as usize {
                target_state[i][j] = (i * n as usize + j) as u16;
            }
        }
        //target_state[(n-1) as usize][(n-1) as usize] = 0;
    }

    for i in 0..n {
        for j in 0..n {
            target_map[target_state[i as usize][j as usize] as usize] = (i, j);
        }
    }
}
