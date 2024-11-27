/*
Solution: BFS

Time: O(q * n ^ 2) | Space: O(n ^ 2)

Runtime: 1 ms | 100.00%
Memory: 2.22 MB | 50.00%

- n: length of 'n'
- q: length of 'queries'
*/

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n: usize = n as usize;
        let mut res: Vec<i32> = Vec::new();

        let mut neighbors: Vec<Vec<usize>> = Vec::new();

        let mut shortest: Vec<i32> = vec![i32::MAX; n];

        for i in 0..n {
            if i < n - 1 {
                neighbors.push(vec![i + 1]);
            } else {
                neighbors.push(Vec::new());
            }
        }

        for i in 0..n {
            shortest[i] = i as i32;
        }

        let mut current: Vec<(usize, usize)> = Vec::new();

        for query in queries.iter() {
            let u: usize = query[0] as usize;
            let v: usize = query[1] as usize;

            neighbors[u].push(v);
            current.push((u, v));


            while let Some((u, v)) = current.pop() {
                if shortest[v] > shortest[u] + 1 {
                    shortest[v] = shortest[u] + 1;
                    for &w in neighbors[v].iter() {
                        current.push((v, w));
                    }

                }
            }

            res.push(shortest[n - 1]);
        }

        res
    }
}