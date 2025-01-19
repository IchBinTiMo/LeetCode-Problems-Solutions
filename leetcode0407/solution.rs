/*
Solution: BFS + Priority Queue

Time: O((m * n) log (m * n)) | Space: O(m * n)

Runtime: 7 ms | 87.10%
Memory: 2.76 MB | 51.61%

- m: length of 'heightMap'
- n: length of 'heightMap[0]'
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        const DIR: [i32; 5] = [0, 1, 0, -1, 0];

        let m: usize = height_map.len();
        let n: usize = height_map[0].len();

        let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();

        let mut visited: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; m];

        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    visited[i][j] = height_map[i][j];
                    heap.push((-height_map[i][j], i, j));
                }
            }
        }

        while let Some((h, i, j)) = heap.pop() {
            for k in 0..4 {
                let new_i: usize = i + DIR[k] as usize;
                let new_j: usize = j + DIR[k + 1] as usize;

                if new_i < m && new_j < n && visited[new_i][new_j] == i32::MAX {
                    visited[new_i][new_j] = -h;
                    heap.push((h.min(-height_map[new_i][new_j]), new_i, new_j));
                } 
            }
        }

        let mut res: i32 = 0;

        for i in 0..m {
            for j in 0..n {
                res += (visited[i][j] - height_map[i][j]).max(0);
            }
        }

        res
    }
}