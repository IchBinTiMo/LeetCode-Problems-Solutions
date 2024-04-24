impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut dp: Vec<i32> = Vec::from([0, 1, 1]);

        for i in 3..=n {
            dp.push(dp[i - 1] + dp[i - 2] + dp[i - 3]);
        }

        dp[n]
    }
}