impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        /// https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/solutions/5228547/god-level-detailed-explanation-2-approaches
        /// 
        /// Time: O(n ^ 2) | Space: O(1)
        /// where n is the length of arr
        let n: usize = arr.len();

        let mut res: usize = 0;

        for i in 0..n {
            let mut xor: i32 = arr[i];
            
            for j in (i + 1)..n {
                xor ^= arr[j];

                if xor == 0 {
                    res += j - i;
                }
            }
        }

        res as i32
    }
}




// impl Solution {
//     pub fn count_triplets(arr: Vec<i32>) -> i32 {
//         /// Brute Force
//         /// 
//         /// Time: O(n ^ 3) | Space: O(n ^ 2)
//         /// where n is the length of arr
//         let n: usize = arr.len();

//         let mut prefix: Vec<Vec<i32>> = vec![vec![-1; n]; n];

//         for i in 0..n {
//             for j in i..n {
//                 if i == j {
//                     prefix[i][j] = arr[i];
//                 } else {
//                     prefix[i][j] = prefix[i][j - 1] ^ arr[j];
//                 }
//             }
//         }

//         let mut res: i32 = 0;

//         for i in 0..n {
//             for j in (i + 1)..n {
//                 for k in j..n {
//                     if prefix[i][j - 1] == prefix[j][k] {
//                         res += 1;
//                     }
//                 }
//             }
//         }

//         res
//     }
// }


