use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        /// Prefix sum -- Optimal
        /// 
        /// Time: O(n) | Space: O(n)
        /// where n is the length of nums
        let n: usize = nums.len();

        if n < 2 {
            return false;
        }

        let mut sum: i32 = 0;

        let mut visited: HashMap<i32, i32> = HashMap::new();

        visited.insert(0, -1);

        for i in 0..n {
            // calculate the remainder of the prefix sum divided by k
            sum += nums[i];
            sum %= k;

            // if the remainde occurs more than once
            //, which means the sum of the subarray from `idx` to `i` is a multiple of k,
            // and the distance between them is greater than 1
            // then the subarray exists            
            if let Some(&prev) = visited.get(&sum) {
                if (i as i32) - prev > 1 {
                    return true;
                }
            } else {
                visited.insert(sum, i as i32);
            }
        }

        false
    }
}

// use std::collections::HashMap;

// impl Solution {
//     pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
//         /// Prefix sum
//         /// 
//         /// Time: O(n) | Space: O(n)
//         /// where n is the length of nums
//         let n: usize = nums.len();

//         if n < 2 {
//             return false;
//         }

//         let mut prefix: Vec<i32> = vec![nums[0] % k];

//         // calculate the remainder of the prefix sum divided by k
//         for i in 1..n {
//             prefix.push((prefix[i - 1] + nums[i]) % k);
//         }

//         let mut visited: HashMap<i32, i32> = HashMap::new();

//         visited.insert(0, -1);

//         // if the remainde occurs more than once
//         //, which means the sum of the subarray from `idx` to `i` is a multiple of k,
//         // and the distance between them is greater than 1
//         // then the subarray exists
//         for i in 0..n {
//             if let Some(idx) = visited.get(&prefix[i]) {
//                 if (i as i32) - idx > 1 {
//                     return true;
//                 }
//             } else {
//                 visited.insert(prefix[i], i as i32);
//             }
//         }

//         false

//     }
// }