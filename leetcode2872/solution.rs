/*
Solution: DFS

Time: O(n) | Space: O(n)

Runtime: 23 ms | 100.00%
Storage: 10.56 MB | 66.67%

*/

impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let n: usize = n as usize;

        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

        let mut visited: Vec<bool> = vec![false; n];

        for edge in edges.iter() {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;

            adj[u].push(v);
            adj[v].push(u);
        }  

        let mut res: i32 = 0;

        let mut sums: Vec<i64> = vec![0; n];

        Self::dfs(&mut res, &mut sums, 0, &adj, &values, &mut visited, k as i64);

        res
    }

    fn dfs(res: &mut i32, sums: &mut Vec<i64>, current: usize, adj: &Vec<Vec<usize>>, values: &Vec<i32>, visited: &mut Vec<bool>, target: i64) {
        visited[current] = true;

        for &next in adj[current].iter() {
            if !visited[next] {
                Self::dfs(res, sums, next, adj, values, visited, target);
                sums[current] += sums[next];
            }
        }

        sums[current] += values[current] as i64;

        if sums[current] % target == 0 {
            *res += 1;
        }
    }
}