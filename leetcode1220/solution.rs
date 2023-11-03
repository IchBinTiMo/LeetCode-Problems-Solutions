use std::collections::HashMap;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut dp: [u32; 5] = [1, 1, 1, 1, 1];
        let mut prev = dp.clone();

        const MOD: u32 = 1_000_000_007;

        for _ in 2..=n {
            for i in 0..prev.len() {
                match i {
                    0 => {
                        dp[i] = (prev[1] + prev[2] + prev[4]) % MOD;
                    }
                    1 => {
                        dp[i] =( prev[0] + prev[2]) % MOD;
                    }
                    2 => {
                        dp[i] = (prev[1] + prev[3]) % MOD;
                    }
                    3 => {
                        dp[i] = (prev[2]) % MOD;
                    }
                    4 => {
                        dp[i] = (prev[2] + prev[3]) % MOD;
                    }
                    _ => {}
                }
            }
            prev = dp.clone();
        }


        (dp.iter().sum::<u32>() % MOD) as i32
    }
}