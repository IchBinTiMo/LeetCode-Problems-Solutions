use std::collections::VecDeque;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        /// BFS
        /// 
        /// Time: O(V + E) | Space: O(V + E)
        if source == destination {
            return true;
        }

        let n: usize = n as usize;
        let mut visited: Vec<bool> = vec![false; n];

        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

        // build graph
        for edge in edges.iter() {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }

        let mut stack: VecDeque<usize> = VecDeque::new();

        stack.push(source as usize);

        // DFS
        while let Some(v) = stack.pop() {

            for &next in graph[v].iter() {
                if !visited[next] {
                    stack.push(next);
                    visited[next] = true;
                }
            }

            // break if destination is visited
            if visited[destination as usize] {
                break;
            }
        }

        visited[destination as usize]
    }
}

// impl Solution {
//     pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
//         /// DFS
//         /// 
//         /// Time: O(V + E) | Space: O(V + E)
//         if source == destination {
//             return true;
//         }

//         let n: usize = n as usize;
//         let mut visited: Vec<bool> = vec![false; n];

//         let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

//         // build graph
//         for edge in edges.iter() {
//             graph[edge[0] as usize].push(edge[1] as usize);
//             graph[edge[1] as usize].push(edge[0] as usize);
//         }

//         let mut stack: Vec<usize> = Vec::new();

//         stack.push(source as usize);

//         // DFS
//         while let Some(v) = stack.pop() {

//             for &next in graph[v].iter() {
//                 if !visited[next] {
//                     stack.push(next);
//                     visited[next] = true;
//                 }
//             }

//             // break if destination is visited
//             if visited[destination as usize] {
//                 break;
//             }
//         }

//         visited[destination as usize]
//     }
// }