/*
Solution: Prefix Sum

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 3.65 MB | 25.00%

- n: length of 'grid[0]'
*/

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n: usize = grid[0].len();

        let mut prefix: Vec<Vec<i64>> = vec![vec![0; n]; 2];

        for i in 0..2 {
            let mut current: i64 = 0;
            for j in 0..n {
                current += grid[i][j] as i64;

                prefix[i][j] = current;
            }
        }

        let mut left: i64 = 0;
        let mut right: i64 = prefix[0][n - 1] - prefix[0][0];

        let mut res: i64 = left.max(right);

        for i in 1..n {
            right = prefix[0][n - 1] - prefix[0][i];
            left = prefix[1][i - 1];

            res = res.min(left.max(right));
        }

        res
    }
}