impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        Solution::backtrack(&mut ans, 1, &mut vec![], n, k);
        ans
    }

    pub fn backtrack(ans: &mut Vec<Vec<i32>>, current: i32, path: &mut Vec<i32>, n: i32, k: i32) {
        if k == 0 {
            ans.push(path.clone());
            return;
        }

        for i in current..=n {
            path.push(i);
            Solution::backtrack(ans, i + 1, path, n, k - 1);
            path.pop();
        }

        return;
    }
}

// impl Solution {
//     pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
//         let mut ans: Vec<Vec<i32>> = Vec::new();
//         Solution::backtrack(&mut ans, 1, &mut vec![], n, k);
//         ans
//     }

//     pub fn backtrack(ans: &mut Vec<Vec<i32>>, current: i32, path: &mut Vec<i32>, n: i32, k: i32) {
//         if k == 0 {
//             ans.push(path.to_vec());
//             return;
//         }

//         for i in current..=n {
//             path.push(i);
//             Solution::backtrack(ans, i + 1, path, n, k - 1);
//             path.pop();
//         }

//         return;
//     }
// }

// // impl Solution {
// //     pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
// //         let mut ans: Vec<Vec<i32>> = Vec::new();
// //         Solution::backtrack(&mut ans, 1, &mut vec![], n, k);
// //         ans
// //     }

// //     pub fn backtrack(ans: &mut Vec<Vec<i32>>, current: i32, path: &mut Vec<i32>, n: i32, k: i32) {
// //         if path.len() == k as usize {
// //             ans.push(path.to_vec());
// //             return;
// //         }

// //         for i in current..=n {
// //             path.push(i);
// //             Solution::backtrack(ans, i + 1, path, n, k);
// //             path.pop();
// //         }

// //         return;
// //     }
// // }