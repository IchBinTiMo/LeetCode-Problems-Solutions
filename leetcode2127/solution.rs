/*
Solution: Topological Sort

Time: O(n) | Space: O(n)

Runtime: 1 ms | 100.00%
Memory: 3.68 MB | 100.00%

- n: length of 'favorite'

ref: https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/solutions/6329688/simple-max-length-cycle
*/

use std::collections::VecDeque;

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n: usize = favorite.len();

        let mut in_degree: Vec<i32> = vec![0; n];
        let mut chain_len: Vec<i32> = vec![0; n];

        let mut visited: Vec<bool> = vec![false; n];

        for &num in favorite.iter() {
            in_degree[num as usize] += 1;
        }

        let mut q: VecDeque<usize> = VecDeque::new();

        for i in 0..n {
            if in_degree[i] == 0 {
                q.push_back(i);
            }
        }

        while let Some(node) = q.pop_front() {
            visited[node] = true;

            let next: usize = favorite[node] as usize;
            chain_len[next] = chain_len[node] + 1;

            in_degree[next] -= 1;

            if in_degree[next] == 0 {
                q.push_back(next);
            }
        }

        let mut max_cycle: i32 = 0;
        let mut total_chains: i32 = 0;

        for i in 0..n {
            if !visited[i] {
                let mut current: usize = i;
                let mut cycle_len: i32 = 0;

                while !visited[current] {
                    visited[current] = true;
                    current = favorite[current] as usize;
                    cycle_len += 1;
                }

                if cycle_len == 2 {
                    total_chains += 2 + chain_len[i] + chain_len[favorite[i] as usize];
                } else {
                    max_cycle = max_cycle.max(cycle_len);
                }
            }
        }

        max_cycle.max(total_chains)
    }
}