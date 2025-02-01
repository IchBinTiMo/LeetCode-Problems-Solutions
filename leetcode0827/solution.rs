/*
Solution: BFS

Time: O(n * n) | Space: O(n * n)

Runtime: 35 ms | 76.92%
Memory: 10.74 MB | 15.38%

- n: length of grid
*/

use std::collections::VecDeque;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n: usize = grid.len();

        let dir: [i32; 5] = [0, 1, 0, -1, 0];

        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; n];
        let mut degree: Vec<Vec<i32>> = vec![vec![0; n]; n];

        let mut parents: Vec<Vec<usize>> = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                parents[i][j] = i * n + j;
            }
        }

        let mut water: Vec<(usize, usize)> = Vec::new();

        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        let mut res: i32 = 0;

        for i in 0..n {
            for j in 0..n {
                if !visited[i][j] && grid[i][j] == 1 {
                    let root: (usize, usize) = (i, j);


                    q.push_back((i, j));

                    visited[i][j] = true;

                    while let Some((r, c)) = q.pop_front() {
                        degree[i][j] += 1;  
                        for k in 0..4 {
                            let new_r: usize = r + dir[k] as usize;
                            let new_c: usize = c + dir[k + 1] as usize;

                            if new_r < n && new_c < n && !visited[new_r][new_c] && grid[new_r][new_c] == 1 {
                                visited[new_r][new_c] = true;
                                parents[new_r][new_c] = i * n + j;
                                q.push_back((new_r, new_c));
                            }

                        }
                    }
                } else if grid[i][j] == 0 {
                    water.push((i, j));
                }

                res = res.max(degree[i][j]);
            }
        }

        while let Some((r, c)) = water.pop() {
            let mut visited_parents: Vec<usize> = Vec::new();
            let mut deg: i32 = 0;

            for k in 0..4 {
                let new_r: usize = r + dir[k] as usize;
                let new_c: usize = c + dir[k + 1] as usize;

                if new_r < n && new_c < n && grid[new_r][new_c] == 1 {
                    let parent: usize = parents[new_r][new_c];

                    let p_r: usize = parent / n;
                    let p_c: usize = parent % n;

                    if !visited_parents.contains(&(p_r * n + p_c)) {
                        deg += degree[p_r][p_c];
                        visited_parents.push(p_r * n + p_c);
                    }
                }

            }
            res = res.max(1 + deg);
        }

        res
    }
}