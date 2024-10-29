/*
Solution: DP

Time: O(mn) | Space: O(mn) (space can be optimized to O(n) I guess)

Runtime: 7 ms | 100.00%
Memory: 3.46 MB | 100.00%

- m: length of 'grid'
- n: length of 'grid[0]'
*/

use std::collections::HashSet;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

        let mut current: HashSet<usize> = HashSet::from_iter(0..m);

        let mut res: i32 = 0;

        let mut c: usize = 0;

        while !current.is_empty() && c < (n - 1) {
            let mut next: HashSet<usize> = HashSet::new();

            for &r in current.iter() {
                if r < m - 1 {
                    if grid[r + 1][c + 1] > grid[r][c] {
                        dp[r + 1][c + 1] = dp[r + 1][c + 1].max(dp[r][c] + 1);
                        res = res.max(dp[r + 1][c + 1]);
                        next.insert(r + 1);
                    }
                }

                if r > 0 {
                    if grid[r - 1][c + 1] > grid[r][c] {
                        dp[r - 1][c + 1] = dp[r - 1][c + 1].max(dp[r][c] + 1);
                        res = res.max(dp[r - 1][c + 1]);
                        next.insert(r - 1);
                    }

                }

                if grid[r][c + 1] > grid[r][c] {
                    dp[r][c + 1] = dp[r][c + 1].max(dp[r][c] + 1);
                    res = res.max(dp[r][c + 1]);
                    next.insert(r);
                }
            }

            c += 1;
            current = next;

        }

        res
    }
}