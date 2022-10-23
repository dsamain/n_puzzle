
use n_puzzle::*;


fn main() {
    let mut closed_set: BTreeMap<Vec<u16>, (u32, Vec<u16>)> = BTreeMap::new(); // .0 = cost, .1 = par
    let mut open_set: BTreeMap<Vec<u16>, (u32, Vec<u16>)> = BTreeMap::new();
    let mut target_state: Vec<u16> = Vec::new();
    let mut target_map: Vec<u16> = Vec::new();
    let mut start: Vec<u16> = Vec::new();
    let mut n: u16 = 0;


    type Binop = fn(&Vec<u16>, &Vec<u16>) -> u32;

    //let mut g: Binop = nothing;
    //let mut h: Binop = misplaced_tiles;
    let mut h: Binop = euclidian_distance_squared;

    parse(&mut n, &mut start, &mut h);

    println!("n = {}", n);
    println!("start = {:?}", start);

    set_target(n, &mut target_state, &mut target_map);
    println!("target_state = {:?}", target_state);

    println!("h(start) = {}", h(&start, &target_map));

    //open_set.insert(start.clone(), (0, start.clone()));
    //open_set.insert(vec![123], (0, start.clone()));

    //for _ in 0..1000000 {
        //open_set.insert(vec![rand::random(), 1], (0, start.clone()));
    //}

    //let mut tmp = open_set.iter();
    ////dbg!(tmp.clone());
    //println!("fisrt entry: {:?}", open_set.iter().next().unwrap());
}
