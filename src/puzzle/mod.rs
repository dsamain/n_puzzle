
// //use std::rc::Rc;
// use std::cmp::Ordering;
// use crate::*;

// #[derive(Clone, PartialEq, Eq)]
// pub struct Puzzle {
//     pub state: Rc<Vec<u16>>,
//     pub par: Option<Rc<Puzzle>>,
//     pub idx: i32,
//     pub cost: i32,
//     pub fcost: i32,
//     pub n: i32,
// }

// impl PartialOrd for Puzzle {
//     fn partial_cmp(&self, other: &Puzzle) -> Option<Ordering> {
//         if self.fcost == other.fcost {
//             return Some(other.cost.cmp(&self.cost));
//         }
//         Some(other.fcost.cmp(&self.fcost))
//     }
// }

// impl Ord for Puzzle {
//     fn cmp(&self, other: &Self) -> Ordering {
//         if self.fcost == other.fcost {
//             return other.cost.cmp(&self.cost);
//         }
//         other.fcost.cmp(&self.fcost)
//     }
// }
