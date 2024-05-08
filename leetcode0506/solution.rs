use std::collections::BinaryHeap;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n: usize = score.len();
        let mut res: Vec<String> = vec![String::new(); n];

        let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

        for i in 0..n {
            heap.push((-score[i], i));
        }

        while let Some((s, idx)) = heap.pop() {
            res[idx] = match heap.len() {
                0 => String::from("Gold Medal"),
                1 => String::from("Silver Medal"),
                2 => String::from("Bronze Medal"),
                _ => format!("{}", heap.len() + 1)
            };
        }

        res
    }
}

// use std::collections::BinaryHeap;
// use std::cmp::Reverse;

// impl Solution {
//     pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
//         let n: usize = score.len();
//         let mut res: Vec<String> = vec![String::new(); n];

//         let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

//         for i in 0..n {
//             heap.push(Reverse((score[i], i)));
//         }

//         while let Some(Reverse((s, idx))) = heap.pop() {
//             res[idx] = match heap.len() {
//                 0 => String::from("Gold Medal"),
//                 1 => String::from("Silver Medal"),
//                 2 => String::from("Bronze Medal"),
//                 _ => format!("{}", heap.len() + 1)
//             };
//         }

//         res
//     }
// }