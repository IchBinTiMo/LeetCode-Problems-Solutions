impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut prev: Vec<Vec<i32>> = vec![vec![-1; n]; n];

        prev[0][n - 1] = grid[0][0] + grid[0][n - 1];

        for i in 1..m {
            let mut dp: Vec<Vec<i32>> = vec![vec![-1; n]; n];

            for j in 0..n {
                for k in (j + 1)..n {

                    for dj in 0..3 {
                        for dk in 0..3 {
                            let prev_val = *prev.get(j + dj - 1).unwrap_or(&vec![-1_i32; n]).get(k + dk - 1).unwrap_or(&(-1));
                            if prev_val != -1 {
                                dp[j][k] = dp[j][k].max(prev_val + grid[i][j] + if j == k {0} else {grid[i][k]});
                            }
                        }
                    }
                }
            }

            prev = dp;
            
        }
        let mut ans: i32 = i32::MIN;

        for j in 0..n {
            for k in 0.. n {
                ans = ans.max(prev[j][k]);
            }
        }

        ans
    }
}