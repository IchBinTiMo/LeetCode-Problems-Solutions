use std::collections::{HashSet, HashMap, VecDeque};

/*
Solution 1:

BFS

Time: O(V + E) | Space: O(V)

The second minimum value must equals to (the smallest value + 1) or (the smallest value + 2).
Therefore, the second time we visit node n, we can get the second minimum value.
*/

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n: usize = n as usize;
        

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
        let mut counts: Vec<Vec<i32>> = vec![vec![i32::MAX, i32::MAX]; n + 1];
        
        counts[1][0] = 0;

        for edge in edges.iter() {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;

            graph[u].push(v);
            graph[v].push(u);

        }

        let mut q: VecDeque<(usize, i32)> = VecDeque::new();

        q.push_back((1, 0));

        while let Some((root, depth)) = q.pop_front() {
            for &neighbor in graph[root].iter() {

                if depth + 1 < counts[neighbor][0] {
                    counts[neighbor][0] = depth + 1;
                    q.push_back((neighbor, depth + 1));
                } else if counts[neighbor][0] < depth + 1 && depth + 1 < counts[neighbor][1] {
                    counts[neighbor][1] = depth + 1;
                    
                    if neighbor == n {
                        q.clear();
                        break;
                    }

                    q.push_back((neighbor, depth + 1));
                }
            }
        }
        let mut edge_cnt: i32 = counts[n][1];

        let mut res: i32 = 0;

        while edge_cnt > 0 {
            res += time;

            if edge_cnt != 1 && ((res / change) & 1) == 1 {
                res = change * (res / change + 1);
            }

            edge_cnt -= 1;
        }

        res
    }
}

/*
Solution 2:

BFS

Time: O(V + E) | Space: O(V + E)
*/

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n: usize = n as usize;
        
        let mut reachable_cnt: HashSet<i32> = HashSet::new();
        let mut edge_cnt: i32 = i32::MIN;
        let mut max_depth: i32 = i32::MIN;

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
        let mut visited: Vec<i32> = vec![0; n + 1];
        visited[1] = 1;

        for edge in edges.iter() {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;

            graph[u].push(v);
            graph[v].push(u);

        }

        let mut q: VecDeque<(usize, i32)> = VecDeque::new();

        q.push_back((1, 0));

        while reachable_cnt.len() < 2 {
            if let Some((root, depth)) = q.pop_front() {
                max_depth = max_depth.max(depth + 1);
                for &neighbor in graph[root].iter() {

                    // idk why the expression "visited[neighbor] <= max_depth / 2 + 2" works
                    if visited[neighbor] <= max_depth / 2 + 2 {
                        q.push_back((neighbor, depth + 1));
                        visited[neighbor] += 1;
                    }

                    if neighbor == n {
                        reachable_cnt.insert(depth + 1);
                        edge_cnt = edge_cnt.max(depth + 1);

                        for v in visited.iter_mut() {
                            *v = 0;
                        }

                        visited[n] = 1;
                    }
                }
            }
        }

        let mut res: i32 = 0;

        while edge_cnt > 0 {
            res += time;

            if edge_cnt != 1 && ((res / change) & 1) == 1 {
                res = change * (res / change + 1);
            }

            edge_cnt -= 1;
        }

        res
    }
}
