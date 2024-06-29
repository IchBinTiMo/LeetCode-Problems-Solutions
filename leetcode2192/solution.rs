impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        /// DFS
        /// 
        /// Time: O(V + E) | Space: O(V + E)
        /// where V is the number of vertices and E is the number of edges
        let n: usize = n as usize;

        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

        for edge in edges {
            let from: usize = edge[0] as usize;
            let to: usize = edge[1] as usize;

            adj[from].push(to);
        }

        let mut res: Vec<Vec<i32>> = vec![Vec::new(); n];

        for i in 0..n {
            Self::DFS(&adj, &mut res, i, i)
        }

        res
    }

    fn DFS(adj: &Vec<Vec<usize>>, res: &mut Vec<Vec<i32>>, parent: usize, child: usize) {
        for &c in adj[child].iter() {
            if res[c].is_empty() || *res[c].last().unwrap() != parent as i32 {
                res[c].push(parent as i32);
                Self::DFS(adj, res, parent, c);
            }
        }
    }
}