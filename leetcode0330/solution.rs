impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        /// Greedy
        /// 
        /// Time O(n) | Space O(1)
        /// where n is the length of nums
        let n : i64 = n as i64;

        let mut res: i32 = 0;

        let mut idx: usize = 0; // the index of the next number to be compared with 'max'

        let mut max: i64 = 0; // the maximum number that can be patched

        while max < n {
            if idx < nums.len() && max + 1 >= nums[idx] as i64 {
                // if the 'idx'th number in nums is less than or equal to 'max' + 1
                // then update 'max' based on the 'idx'th number of nums
                max += nums[idx] as i64;
                idx += 1;
            } else {
                // otherwise, patch 'max' + 1
                res += 1;
                max += max + 1;
            }
        }

        res
    }
}

// impl Solution {
//     pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
//         let n : i64 = n as i64;
//         let mut acc: Vec<i64> = Vec::new();

//         let mut idx: usize = 0;

//         let mut max: i64 = 0;

//         while max < n {
//             while idx < nums.len() && max + 1 >= nums[idx] as i64 {
//                 acc.push(nums[idx] as i64);
//                 max += nums[idx] as i64;
//                 idx += 1;
//             }

//             if max >= n {
//                 break;
//             }

//             acc.push(max + 1);

//             max += max + 1;
//         }

//         for i in idx..nums.len() {
//             acc.push(nums[i] as i64);
//         }

//         (acc.len() - nums.len()) as i32

        
//     }
// }