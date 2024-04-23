use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        /// BFS
        /// 
        /// Time: O(n) | Space: O(n)
        if n == 1 {
            return vec![0];
        }
        let n: usize = n as usize;
        let neighbors: Vec<Vec<usize>> = edges.iter()
                                            .fold(vec![Vec::new(); n], |mut acc, edge| 
                                                {
                                                    acc[edge[0] as usize].push(edge[1] as usize);
                                                    acc[edge[1] as usize].push(edge[0] as usize);
                                                    acc
                                                });

        let mut visited: Vec<bool> = vec![false; n];

        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut deepest: usize = usize::MAX;

        queue.push_back(edges[0][0] as usize);

        // bfs to find the deepest node
        while let Some(current) = queue.pop_front() {
            if !visited[current] {
                visited[current] = true;
                deepest = current;

                for &neighbor in neighbors[current].iter() {
                    queue.push_back(neighbor);
                }
            }
        }

        let mut visited: Vec<bool> = vec![false; n];

        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();

        let mut parents: Vec<usize> = vec![usize::MAX; n];

        let mut length: i32 = 0;

        queue.push_back((deepest, usize::MAX, 0));

        // bfs to find the farthest node from the previous deepest node
        while let Some((current, prev, dist)) = queue.pop_front() {
            if !visited[current] {
                visited[current] = true;
                parents[current] = prev;
                deepest = current;
                length = dist;

                for &neighbor in neighbors[current].iter() {
                    queue.push_back((neighbor, current, dist + 1));
                }
            }
        }

        // move to the middle of the two nodes we just got
        for _ in 0..(length / 2) {
            deepest = parents[deepest];
        }

        // length is even means the number of nodes on the longest path is odd so there is one center point
        // otherwise there are two center points
        if length % 2 == 0 { 
            vec![deepest as i32]
        } else { // 
            vec![deepest as i32, parents[deepest] as i32]
        }

    }
}