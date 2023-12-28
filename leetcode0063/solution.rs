impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

        dp[0][0] = 1;

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                } else {
                    if obstacle_grid[i][j] == 1 {
                        continue;
                    }
                    if i == 0 {
                        dp[i][j] = dp[i][j - 1];
                    } else if j == 0 {
                        dp[i][j] = dp[i - 1][j];
                    } else {
                        dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                    }
                }
            }
        }
        dp[m - 1][n - 1]
    }
}