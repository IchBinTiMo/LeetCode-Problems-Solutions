use std::collections::VecDeque;
use std::collections::HashSet;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        if stones[1] != 1 {
            return false;
        }
        let l = stones.len();

        if l == 2 {
            return true;
        }

        let mut dp: VecDeque<(i32, i32)> = VecDeque::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        dp.push_back((1, 1));

        while let Some((step, des)) = dp.pop_front() {
            if visited.get(&(step, des)).is_some() {
                continue;
            }

            visited.insert((step, des));

            for k in -1..=1 {
                if step + k <= 0 {
                    continue;
                }

                if step + k + des == stones[l - 1] {
                    return true;
                }
                
                if stones.contains(&(step + k + des)) {
                    dp.push_back((step + k, step + k + des));
                }


            }
        }

        false
    }
}

// impl Solution {
//     pub fn can_cross(stones: Vec<i32>) -> bool {
//         if stones[1] != 1 {
//             return false;
//         }
//         let l = stones.len();

//         if l == 2 {
//             return true;
//         }

//         let mut dp: Vec<Vec<bool>> = vec![vec![false; l]; l];

//         dp[1][0] = true;

//         for i in 1..l {
//             for j in 0..i {
//                 if dp[i][j] == false {
//                     continue;
//                 }
//                 let current = stones[i] - stones[j];

//                 for k in -1..=1 {
//                     if current + k <= 0 {
//                         continue;
//                     }

//                     if let Some(index) = stones.iter().position(|&e| e == current + k + stones[i]) {
//                         if index == l - 1 {
//                             return true;
//                         }
//                         dp[index][i] = true;
//                     }

//                 }
//             }
//         }

//         false
//     }
// }
// // impl Solution {
// //     pub fn can_cross(stones: Vec<i32>) -> bool {
// //         if stones[1] != 1 {
// //             return false;
// //         }
// //         let l = stones.len();

// //         let mut dp: Vec<Vec<bool>> = vec![vec![false; l]; l];

// //         dp[1][0] = true;

// //         for i in 1..l {
// //             for j in 0..i {
// //                 if dp[i][j] == false {
// //                     continue;
// //                 }
// //                 let current = stones[i] - stones[j];

// //                 for k in -1..=1 {
// //                     if current + k <= 0 {
// //                         continue;
// //                     }

// //                     if let Some(index) = stones.iter().position(|&e| e == current + k + stones[i]) {
// //                         dp[index][i] = true;
// //                     }

// //                 }
// //             }
// //         }

// //         dp[l - 1].iter().any(|&v| v)
// //     }
// // }