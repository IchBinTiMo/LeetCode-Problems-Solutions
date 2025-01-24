/*
Solution: DFS

Time: O(n) | Space: O(n)

Runtime: 5 ms | 44.44%
Memory: 3.14 MB | 66.67%

- n: length of 'graph'
*/

use std::collections::VecDeque;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n: usize = graph.len();

        let mut safe: Vec<bool> = vec![false; n];
        let mut visited: Vec<bool> = vec![false; n];

        let mut q: VecDeque<usize> = VecDeque::new();

        for i in 0..n {
            if graph[i].is_empty() {
                safe[i] = true;
                visited[i] = true;
            } else {
                q.push_back(i);
            }
        }

        while let Some(u) = q.pop_front() {
            Self::dfs(&mut safe, u, &mut visited, &graph);
        }

        let mut res: Vec<i32> = Vec::new();

        for i in 0..n {
            if safe[i] {
                res.push(i as i32);
            }
        }

        res
    }

    fn dfs(safe: &mut Vec<bool>, current: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<i32>>) {
        let mut valid: bool = true;

        visited[current] = true;
        for &v in graph[current].iter() {
            let v: usize = v as usize;
            if !visited[v] {
                visited[v] = true;
                Self::dfs(safe, v, visited, graph);
            }

            valid &= safe[v]
        }

        safe[current] = valid;
    }
}