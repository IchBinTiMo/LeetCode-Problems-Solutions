impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {

        let mut ans: i64 = 0;

        const MOD: i64 = 1_000_000_007;

        let mut prev: Vec<Vec<i64>> = vec![vec![0; (n + 2) as usize]; (m + 2) as usize];

        prev[(start_row + 1) as usize][(start_column + 1) as usize] = 1;

        for mv in 0..max_move {
            let mut dp: Vec<Vec<i64>> = vec![vec![0; (n + 2) as usize]; (m + 2) as usize];
            for r in 1..=m {
                for c in 1..=n {
                    
                    let r = r as usize;
                    let c = c as usize;

                    if r == 1 as usize {
                        ans = (ans + prev[r][c]) % MOD;
                    }

                    if r == m as usize {
                        ans = (ans + prev[r][c]) % MOD;
                    }

                    if c == 1 as usize {
                        ans = (ans + prev[r][c]) % MOD;
                    }

                    if c == n as usize {
                        ans = (ans + prev[r][c]) % MOD;
                    }

                    dp[r][c] += (prev[r - 1][c] + 
                        prev[r + 1][c] + 
                        prev[r][c - 1] + 
                        prev[r][c + 1]) % MOD;
                }
            }
            prev = dp;
        }



        ans as i32
    }
}