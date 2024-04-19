impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        /// DFS
        let mut res: i32 = 0;

        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

        let mut stack: Vec<(usize, usize)> = Vec::new();


        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' && !visited[i][j] {
                    res += 1;
                    stack.push((i, j));
                    visited[i][j] = true;

                    while let Some((r, c)) = stack.pop() {
                        for (new_r, new_c) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                            if new_r < m && new_c < n && grid[new_r][new_c] == '1' && !visited[new_r][new_c] {
                                visited[new_r][new_c] = true;
                                stack.push((new_r, new_c));
                            }
                        }
                    }

                }
            }
        }
        
        res
    }
}