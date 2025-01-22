/*
Solution: BFS

Time: O(m * n) | Space: O(m * n)

Runtime: 11 ms | 100.00%
Memory: 14.51 MB | 28.57%

- m: length of 'is_water'
- n: length of 'is_water[0]'
*/

use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m: usize = is_water.len();
        let n: usize = is_water[0].len();

        let dir: [i32; 5] = [0, 1, 0, -1, 0];

        let mut res: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; m];

        let mut q: VecDeque<(i32, usize, usize)> = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    res[i][j] = 0;

                    q.push_back((0, i, j));
                }
            }
        }

        while let Some((height, i, j)) = q.pop_front() {
            for k in 0..4 {
                let new_i: usize = i + dir[k] as usize;
                let new_j: usize = j + dir[k + 1] as usize;

                if new_i < m && new_j < n && res[new_i][new_j] > height + 1 {
                    res[new_i][new_j] = height + 1;

                    q.push_back((res[new_i][new_j], new_i, new_j));
                }
            }
        }

        res
    }
}