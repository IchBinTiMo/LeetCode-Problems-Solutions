/*
Solution: BFS

Time: O(m * n) | Space: O(m * n)

Runtime: 3 ms | 66.67%
Memory: 2.24 MB | 100.00%

- m: length of 'grid'
- n: length of 'grid[0]'
*/

use std::collections::VecDeque;

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut visited: i128 = 0;

        let mut res: i32 = 0;

        let dir: [i32; 5] = [0, 1, 0, -1, 0];

        for i in 0..m {
            for j in 0..n {

                if grid[i][j] > 0 && (visited & (1 << (n * i + j))) == 0 {
                    let mut current: i32 = 0;

                    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

                    q.push_back((i, j));
                    visited |= 1 << (n * i + j);

                    while let Some((r, c)) = q.pop_front() {

                        current += grid[r][c];

                        for k in 0..4 {
                            let new_r: usize = r + dir[k] as usize;
                            let new_c: usize = c + dir[k + 1] as usize;

                            if new_r < m && new_c < n && grid[new_r][new_c] > 0 && (visited & (1 << (n * new_r + new_c))) == 0 {
                                visited |= 1 << (n * new_r + new_c);
                                q.push_back((new_r, new_c));
                            }

                        }
                    }

                    res = res.max(current);
                }
            }
        }

        res
    }
}