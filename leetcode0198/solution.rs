impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut n: usize = nums.len();

        let mut dp: Vec<i32> = vec![0; n + 2];

        for i in 0..n {
            dp[i + 2] = dp[i + 1].max(dp[i] + nums[i]);
        }

        *dp.last().unwrap()
    }
}