/*
Solution: BFS

Time: O((m * n) log (m * n)) | Space: O(m * n)

Runtime: 8 ms | 18.18%
Memory: 2.62 MB | 45.45%
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        let mut costs: Vec<Vec<i32>> = vec![vec![i32::MIN; n]; m];

        let mut res: i32 = i32::MAX;

        costs[0][0] = 0;
        heap.push((0, 0, 0));

        while let Some((cost, i, j)) = heap.pop() {
            if i == m - 1 && j == n - 1 {
                res = res.min(-cost);
            } else {

                let dir: [i32; 5] = [0, 1, 0, -1, 0];

                for k in 0..4 {
                    let new_i: usize = i + dir[k] as usize;
                    let new_j: usize = j + dir[k + 1] as usize;

                    if new_i < m && new_j < n {
                        if grid[i][j] == 1 && (new_i == i && new_j == j + 1) || 
                            grid[i][j] == 2 && (new_i == i && new_j == j - 1) || 
                            grid[i][j] == 3 && (new_i == i + 1 && new_j == j) ||
                            grid[i][j] == 4 && (new_i == i - 1 && new_j == j) {
                                if cost > costs[new_i][new_j] {
                                    costs[new_i][new_j] = cost;
                                    heap.push((cost, new_i, new_j));
                                }
                        } else {
                            if cost - 1 > costs[new_i][new_j] {
                                costs[new_i][new_j] = cost - 1;
                                heap.push((cost - 1, new_i, new_j));
                            }
                        }

                    }

                }
            }
        }

        res
    }
}