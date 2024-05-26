impl Solution {
    pub fn check_record(n: i32) -> i32 {
        /// Dynamic Programming
        /// 
        /// Time O(N) | Space O(N)
        let n: usize = n as usize;
        let MOD = 1_000_000_007;

        let mut dp: Vec<[i64; 6]> = vec![[0; 6]; n + 1];
        
        dp[1] = [1, 1, 0, 1, 0, 0];

        for i in 2..=n {
            dp[i][0] = (0..3).fold(0, |acc, j| acc + dp[i - 1][j]) % MOD; // there is no 'A' in the record and the last element is not 'L'
            dp[i][1] = dp[i - 1][0]; // there is no 'A' in the record and the last element is 'L' while the element before last is not 'L'
            dp[i][2] = dp[i - 1][1]; // there is no 'A' in the record and the last 2 elements are 'L'
            dp[i][3] = (0..6).fold(0, |acc, j| acc + dp[i - 1][j]) % MOD; // there is 'A' in the record and the last element is not 'L'
            dp[i][4] = dp[i - 1][3]; // there is 'A' in the record and the last element is 'L' while the element before last is not 'L'
            dp[i][5] = dp[i - 1][4]; // there is 'A' in the record and the last 2 elements are 'L'
        }

        ((0..6).fold(0, |acc, j| acc + dp[n][j]) % MOD) as i32
    }
}