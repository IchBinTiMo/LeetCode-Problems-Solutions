/*
Solution: Topological Sort

Time: O(n) | Space: O(n * n)

Runtime: 1 ms | 100.00%
Memory: 3.12 MB | 100.00%

- n: length of 'num_courses'
*/

use std::collections::VecDeque;

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n: usize = num_courses as usize;
        let mut prevs: Vec<u128> = vec![0; n];

        let mut in_degree: Vec<i32> = vec![0; n];
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

        for pre in prerequisites.iter() {
            let u: usize = pre[0] as usize;
            let v: usize = pre[1] as usize;

            adj[u].push(v);

            in_degree[v] += 1;
        }

        let mut q: VecDeque<usize> = VecDeque::new();

        for i in 0..n {
            if in_degree[i] == 0 {
                q.push_back(i);
            }
        }

        while let Some(current) = q.pop_front() {
            while let Some(next) = adj[current].pop() {
                prevs[next] |= 1 << current ;
                prevs[next] |= prevs[current];

                in_degree[next] -= 1;

                if in_degree[next] == 0 {
                    q.push_back(next);
                }
                
            }
        }

        let mut res: Vec<bool> = Vec::new();

        for query in queries {
            let u: u128 = query[0] as u128;
            let v: usize = query[1] as usize;

            res.push((prevs[v] & (1 << u)) != 0);
        }

        res
    }
}