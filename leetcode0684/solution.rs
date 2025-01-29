/*
Solution: union find

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.82 MB | 66.67%

- n: length of nodes
*/

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n: usize = edges.len();
        let mut roots: Vec<usize> = (0..=n).map(|idx| idx).collect::<Vec<usize>>();
        let mut degree: Vec<i32> = vec![1; n + 1];

        let mut redundant: Vec<Vec<i32>> = Vec::new();

        for edge in edges.iter() {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;

            if roots[u] == roots[v] {
                redundant.push(vec![u as i32, v as i32]);
            } else {
                if degree[roots[u]] >= degree[roots[v]] {
                    let root: usize = roots[v];
                    for i in 0..=n {
                        if roots[i] == root {
                            degree[roots[i]] -= 1;
                            roots[i] = roots[u];
                            degree[roots[i]] += 1;
                        }
                    }
                } else {
                    let root: usize = roots[u];
                    for i in 0..=n {
                        if roots[i] == root {
                            degree[roots[i]] -= 1;
                            roots[i] = roots[v];
                            degree[roots[i]] -= 1;
                        }
                    }

                }
            }
        }

        redundant.pop().unwrap()
    }
}