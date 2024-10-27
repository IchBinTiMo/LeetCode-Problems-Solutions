/*
Solution: DP

Time: O(m * n) | Space: O(m * n)

Runtime: 0 ms | 100.00%
Memory: 2.92 MB | 33.33%

- m: number of rows
- n: number of columns
*/

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let m: usize = matrix.len();
        let n: usize = matrix[0].len();

        // dp[i][j] is the number of squares with bottom right at (i, j) the grid is in
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

        let mut res: i32 = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    continue;
                }

                if i == 0 || j == 0 {
                    dp[i][j] = matrix[i][j];
                } else {
                    dp[i][j] = 1 + dp[i][j - 1].min(dp[i - 1][j].min(dp[i - 1][j - 1]));
                }

                res += dp[i][j];
            }
        }

        res
    }
}