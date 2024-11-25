/*
Solution: BFS

Time: O(1) | Space: O(n ^ 2)

Runtime: 1 ms | 100.00%
Memory: 2.22 MB | 50.00%

- n: # of possible board
*/

use std::collections::HashSet;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut res: i32 = i32::MAX;

        let mut visited: HashSet<Vec<Vec<i32>>> = HashSet::new();

        let mut q: Vec<(usize, usize, Vec<Vec<i32>>)> = Vec::new();

        let mut current: i32 = 0;

        let dir: [i32; 5] = [0, 1, 0, -1, 0];

        visited.insert(vec![vec![1, 2, 3], vec![4, 5, 0]]);
        q.push((1, 2, vec![vec![1, 2, 3], vec![4, 5, 0]]));

        while !q.is_empty() {
            let mut next: Vec<(usize, usize, Vec<Vec<i32>>)> = Vec::new();

            while let Some((r, c, mut start)) = q.pop() {
                if start == board {
                    if res > current {
                        res = current;
                    }
                    return res;
                } else {
                    for i in 0..4 {
                        let new_r: usize = r + dir[i] as usize;
                        let new_c: usize = c + dir[i + 1] as usize;

                        if new_r < 2 && new_c < 3 {
                            start[r][c] ^= start[new_r][new_c];
                            start[new_r][new_c] ^= start[r][c];
                            start[r][c] ^= start[new_r][new_c];

                            if visited.get(&start).is_none() {
                                visited.insert(start.clone());
                                next.push((new_r, new_c, start.clone()));
                            }

                            start[r][c] ^= start[new_r][new_c];
                            start[new_r][new_c] ^= start[r][c];
                            start[r][c] ^= start[new_r][new_c];
                        }
                    }
                }
            }

            q = next;
            current += 1;
        }

        -1
    }
}
