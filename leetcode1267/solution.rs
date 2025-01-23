/*
Solution: Brute Force

Time: O(m * n) | Space: O(m + n)

Runtime: 0 ms | 100.00%
Memory: 2.78 MB | 33.33%

- m: length of 'grid'
- n: length of 'grid[0]'
*/

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut rows: Vec<i32> = vec![0; m];
        let mut cols: Vec<i32> = vec![0; n];

        let mut res: i32 = 0;

        for i in 0..m {
            for j in 0..n {
                rows[i] += grid[i][j];
                cols[j] += grid[i][j];
            }

            res += rows[i];
        }

        for i in 0..m {
            for j in 0..n {
                if rows[i] * cols[j] == 1 && grid[i][j] == 1 {
                    res -= 1;
                }
            }
        }

        res
    }
}