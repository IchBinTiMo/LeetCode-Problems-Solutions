impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {

        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

        dp[0][0] = grid[0][0];


        for i in 0..m {
            for j in 0..n {
                match (i, j) {
                    (0, 0) => continue,
                    (0, _) => dp[i][j] += grid[i][j] + dp[i][j - 1],
                    (_, 0) => dp[i][j] += grid[i][j] + dp[i - 1][j],
                    (_, _) => dp[i][j] += grid[i][j] + dp[i][j - 1].min(dp[i - 1][j])
                }
            }
        }

        dp[m - 1][n - 1]
    }
}