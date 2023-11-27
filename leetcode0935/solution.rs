impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let mut dp = [1; 10];
        let MOD = 1_000_000_007;

        for _ in 1..n {
            dp = [
                (dp[4] + dp[6]) % MOD,
                (dp[6] + dp[8]) % MOD,
                (dp[7] + dp[9]) % MOD,
                (dp[4] + dp[8]) % MOD,
                ((dp[0] + dp[3]) % MOD + dp[9]) % MOD,
                0,
                ((dp[0] + dp[1]) % MOD + dp[7]) % MOD,
                (dp[2] + dp[6]) % MOD,
                (dp[1] + dp[3]) % MOD,
                (dp[2] + dp[4]) % MOD
            ];
        }

        dp.into_iter().fold(0,|acc,x| (acc + x) % MOD)
    }
}