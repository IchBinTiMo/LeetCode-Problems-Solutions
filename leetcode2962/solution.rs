impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut freq: i32 = 0; // the frequency of the max element
        let mut arr_count: usize = 0; // the number of subarrays where the max element appears less than k times

        let n: usize = nums.len();

        let mut left: usize = 0;

        // find the max element
        let mut max: i32 = *nums.iter().max().unwrap();

        // find the number of subarrays where the max element appears less than k times
        for i in 0..n {
            if nums[i] == max {
                freq += 1;
            }

            while freq >= k {
                if nums[left] == max {
                    freq -= 1;
                }

                left += 1;
            }

            arr_count += i - left + 1;
        }

        // subtract the number of subarrays where the max element appears less than k times from the total number of non-empty subarrays
        let res: i64 = ((n * (n + 1) / 2) - arr_count) as i64;

        res
    }
}

// impl Solution {
//     pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
//         let mut freq: i32 = 0; // the frequency of the max element
//         let mut arr_count: usize = 0; // the number of subarrays where the max element appears less than k times

//         let n: usize = nums.len();

//         let mut left: usize = 0;
//         let mut right: usize = 0;

//         let mut max: i32 = i32::MIN;

//         // find the max element
//         for &num in nums.iter() {
//             max = max.max(num);
//         }

//         // find the number of subarrays where the max element appears less than k times
//         while right < n {
//             if nums[right] == max {
//                 freq += 1;
//             }
//             right += 1;

//             while left < right && freq >= k {
//                 if nums[left] == max{
//                     freq -= 1;
//                 }
//                 left += 1;
//             }
//             arr_count += right - left;
//         }

//         // subtract the number of subarrays where the max element appears less than k times from the total number of non-empty subarrays
//         let res: i64 = ((n * (n + 1) / 2) - arr_count) as i64;

//         res
//     }
// }