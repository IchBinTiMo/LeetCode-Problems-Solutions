impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        /// Brute Force
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut res: i32 = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    res += 4;
                    if i > 0 && grid[i - 1][j] == 1 {
                        res -= 2;
                    }

                    if j > 0 && grid[i][j - 1] == 1 {
                        res -= 2;
                    }
                }
            }
        }

        res

        
    }
}

// use std::collections::VecDeque;

// impl Solution {
//     pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
//         /// BFS
//         let m: usize = grid.len();
//         let n: usize = grid[0].len();

//         let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

//         let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

//         let mut res: i32 = 0;

//         for i in 0..m {
//             for j in 0..n {
//                 if grid[i][j] == 1 && !visited[i][j] {
//                     queue.push_back((i, j));
//                     visited[i][j] = true;


//                     while let Some((r, c)) = queue.pop_front() {
//                         res += 4;

//                         for (new_r, new_c) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
//                             if new_r >= m || new_c >= n || grid[new_r][new_c] == 0 {
//                                 continue;
//                             }
//                             if visited[new_r][new_c] {
//                                 res -= 1;
//                                 continue;
//                             }
//                             res -= 1;
//                             visited[new_r][new_c] = true;
//                             queue.push_back((new_r, new_c));
//                         }

//                     }
//                 }
//             }
//         }

//         res

        
//     }
// }