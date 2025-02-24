/*
Solution: DFS, BFS

Time: O(n) | Space: O(n)

Runtime: 39 ms | 100.00%
Memory: 18.85 MB | 100.00%

- n: length of 'amount'
*/

use std::collections::HashSet;

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, mut amount: Vec<i32>) -> i32 {
        let n: usize = amount.len();
        let bob: usize = bob as usize;

        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];

        let mut set_a: HashSet<usize> = HashSet::new();
        let mut set_b: HashSet<usize> = HashSet::new();

        let mut cost: Vec<i32> = vec![0; n];
        let mut visited: Vec<bool> = vec![false; n];


        let mut res: i32 = i32::MIN;

        for edge in edges {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;

            adj[u].push(v);
            adj[v].push(u);
        }

        let mut path: Vec<usize> = Vec::new();
        let mut tmp: Vec<usize> = Vec::new();

        // dfs to find the path from bob to 0
        Self::dfs(&mut path, &mut tmp, 0, bob, &mut visited, &adj);

        set_a.clear();

        let mut visited: Vec<bool> = vec![false; n];

        set_a.insert(0);
        set_b.insert(bob);

        cost[0] = amount[0];

        visited[0] = true;

        while !set_a.is_empty() {
            let mut nexts_a: HashSet<usize> = HashSet::new();

            if let Some(current) = path.pop() {
                if set_a.contains(&current) {
                    cost[current] -= amount[current] / 2;
                }

                amount[current] = 0;
            }

            for &current in set_a.iter() {
                let mut is_leaf: bool = true;
                for &next in adj[current].iter() {
                    if !visited[next] {
                        is_leaf = false;
                        cost[next] = cost[current] + amount[next];
                        nexts_a.insert(next);
                        visited[next] = true;
                    }
                }
                if is_leaf {
                    res = res.max(cost[current]);
                }

            }

            set_a = nexts_a;
        }

        res
    }

    // dfs to find the path from bob to 0
    fn dfs(res: &mut Vec<usize>, path: &mut Vec<usize>, current: usize, bob: usize, visited: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
        path.push(current);
        
        if current == bob {
            *res = (*path).clone();
            return;
        } else {
            for &next in adj[current].iter() {
                if !visited[next] {
                    visited[next] = true;
                    Self::dfs(res, path, next, bob, visited, adj);
                    path.pop();
                }
            }
        }
    }
}