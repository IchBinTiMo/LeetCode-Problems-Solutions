impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        /// Hash
        /// 
        /// Time: O(n) | Space: O(n)
        let mut visited: Vec<bool> = vec![false; 2001];

        let mut res: i32 = -1;

        for num in nums {
            visited[(1000 + num) as usize] = true;

            if visited[(1000 - num) as usize] {
                if num > 0 {
                    res = res.max(num);
                } else {
                    res = res.max(-num);
                }
            }
        }

        res
    }
}

// impl Solution {
//     pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
//         /// Two Pointers
//         /// 
//         /// Time: O(n log n) | Space: O(1)
//         nums.sort_unstable();

//         let mut left: usize = 0;
//         let mut right: usize = nums.len() - 1;

//         while left < right {
//             if nums[left] == -nums[right] {
//                 return nums[right];
//             } else if nums[left] < -nums[right] {
//                 left += 1;
//             } else {
//                 right -= 1;
//             }
//         }

//         -1
//     }
// }