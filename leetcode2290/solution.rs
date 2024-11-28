/*
Solution: BFS

Time: O(m * n log m * n) | Space: O(m * n)

Runtime: 83 ms | 25.00%
Memory: 13.03 MB | 25.00%

- m: length of 'grid'
- n: length of 'grid[0]'
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();

        let n: usize = grid[0].len();

        let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        let mut visited: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; m];

        let mut res: i32 = i32::MAX;

        let dir: [i32; 5] = [0, 1, 0, -1, 0];

        heap.push((0, 0, 0));

        visited[0][0] = 0;

        while let Some((obs, r, c)) = heap.pop() {
            if r == m - 1 && c == n - 1 {
                res = res.min(visited[r][c]);
                if res == 0 {
                    return res;
                }
                continue;
            }

            for i in 0..4 {
                let new_r = r + dir[i] as usize;
                let new_c = c + dir[i + 1] as usize;
                

                if new_r < m && new_c < n {
                    if visited[new_r][new_c] > grid[new_r][new_c] - obs {
                        visited[new_r][new_c] = visited[new_r][new_c].min(grid[new_r][new_c] - obs);
                        heap.push((obs - grid[new_r][new_c], new_r, new_c));
                    }

                }

            }
        }

        res
    }
}