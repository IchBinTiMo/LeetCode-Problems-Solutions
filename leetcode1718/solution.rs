/*
Solution: Backtrack	

Time: O(2 ^ n + n!) | Space: O(n ^ 2)

Runtime: 0 ms | 100.00%
Memory: 2.19 MB | 100.00%
*/

use std::collections::HashSet;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let n: usize = n as usize;

        let mut res: Vec<i32> = Vec::new();
        let mut path: Vec<i32> = vec![-1; 2 * n - 1];
        let mut visited: HashSet<usize> = HashSet::new();

        Self::backtrack(&mut res, &mut path, 0, n, &mut visited);

        res
    }

    fn backtrack(res: &mut Vec<i32>, path: &mut Vec<i32>, current: usize, n: usize, visited: &mut HashSet<usize>) {
        if visited.len() == n {
            *res = path.clone();
            return;
        } else if current >= path.len() {
            return;
        } else {
            if path[current] != -1 {
                Self::backtrack(res, path, current + 1, n, visited);
            } else {
                for i in (1..=n).rev() {
                    if !res.is_empty() {
                        return;
                    }


                    if visited.contains(&i) {
                        continue;
                    }

                    if i == 1 {
                        path[current] = i as i32;
                        visited.insert(i);
                        
                        Self::backtrack(res, path, current + 1, n, visited);

                        path[current] = -1;
                        visited.remove(&i);
                    } else {
                        if current + i < path.len() && path[current] == -1 && path[current + i] == -1 {
                            path[current] = i as i32;
                            path[current + i] = i as i32;

                            visited.insert(i);

                            Self::backtrack(res, path, current + 1, n, visited);

                            path[current] = -1;
                            path[current + i] = -1;

                            visited.remove(&i);
                        }
                    }
                }
            }
        }
    }
}