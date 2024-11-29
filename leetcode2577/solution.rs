/*
Solution: BFS

Time: O(m * n log m * n) | Space: O(m * n)

Runtime: 58 ms | 50.00%
Memory: 4.10 MB | 100.00%

- m: length of 'grid'
- n: length of 'grid[0]'
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
        let mut res: i32 = i32::MAX;

        heap.push((0, 0, 0));
        visited[0][0] = true;

        let dir: [i32; 5] = [0, 1, 0, -1, 0];

        while let Some((step, r, c)) = heap.pop() {
            if r == m - 1 && c == n - 1 {
                return -step;
            } else {
                for i in 0..4 {
                    let new_r: usize = r + dir[i] as usize;
                    let new_c: usize = c + dir[i + 1] as usize;

                    if new_r < m && new_c < n && !visited[new_r][new_c] {
                        visited[new_r][new_c] = true;
                        let diff: i32 = grid[new_r][new_c] + step;

                        if r + c == 0 && grid[0][1] > 1 && grid[1][0] > 1 {
                            continue;
                        } else {
                            if diff > 0 {
                                if diff & 1 == 1 {
                                    heap.push((-grid[new_r][new_c], new_r, new_c));
                                } else {
                                    heap.push((-(grid[new_r][new_c] + 1), new_r, new_c));
                                }
                            } else {
                                heap.push((step - 1, new_r, new_c));
                            }
                        }
                    }

                }
            }
        }
        
        -1
    }
}