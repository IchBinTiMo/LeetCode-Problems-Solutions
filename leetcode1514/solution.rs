/*
Solution: Dijkstra

Time: O(n * E) | Space: O(n)

Runtime: 16 ms | 100.00%
Memory: 3.64 MB | 100.00%

- n: number of nodes
- E: number of edges
*/

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let start_node: usize = start_node as usize;
        let end_node: usize = end_node as usize;
        
        let mut probs: Vec<f64> = vec![0.0; n as usize];

        probs[start_node] = 1.0;

        for i in 0..(n - 1) {
            let mut updated: bool = false;

            for i in 0..edges.len() {
                let u: usize = edges[i][0] as usize;
                let v: usize = edges[i][1] as usize;
                let p: f64 = succ_prob[i];

                if probs[u] * p > probs[v] {
                    probs[v] = probs[u] * p;
                    updated = true;
                }

                if probs[v] * p > probs[u] {
                    probs[u] = probs[v] * p;
                    updated = true;
                }
            }

            if !updated {
                break;
            }
        }

        probs[end_node]
    }
}