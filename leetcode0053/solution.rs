impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut dp: Vec<i32> = vec![i32::MIN; n];
        let mut ans: i32 = nums[0];

        dp[0] = nums[0];

        for i in 1..n {
            dp[i] = nums[i].max(dp[i - 1] + nums[i]);
            ans = ans.max(dp[i]);
        }

        ans
    }
}

// impl Solution {
//     pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//         let n: usize = nums.len();
//         let mut dp: Vec<i32> = vec![i32::MIN; n];
//         let mut ans: i32 = nums[0];

//         dp[0] = nums[0];

//         for i in 1..n {
//             dp[i] = nums[i].max((dp[i - 1] + nums[i]).max(nums[i]));
//             ans = ans.max(dp[i]);
//         }

//         ans
//     }
// }

// // impl Solution {
// //     pub fn max_sub_array(nums: Vec<i32>) -> i32 {
// //         let n: usize = nums.len();
// //         let mut dp: Vec<i32> = vec![i32::MIN; n];

// //         dp[0] = nums[0];

// //         for i in 1..n {
// //             dp[i] = nums[i].max((dp[i - 1] + nums[i]).max(nums[i]));
// //         }

// //         let mut ans: i32 = i32::MIN;

// //         while let Some(val) = dp.pop() {
// //             ans = ans.max(val);
// //         }

// //         ans
// //     }
// // }