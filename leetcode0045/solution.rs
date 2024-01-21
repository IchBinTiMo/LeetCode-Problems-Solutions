impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut dp: Vec<i32> = vec![i32::MAX; n];

        dp[0] = 0;

        for i in 0..n {
            for j in 1..=nums[i] {
                let j = j as usize;
                if i + j >= n {
                    break;
                }
                dp[i + j] = dp[i + j].min(dp[i] + 1);
            }
        }


        *dp.last().unwrap()

    }
}