impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        /// brute force
        /// 
        /// Time O(N) | Space O(1)
        let mut res: Vec<Vec<i32>> = Vec::new();

        let m: usize = land.len();
        let n: usize = land[0].len();

        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                // if the cell is land and not visited
                // starting from the cell, we go all the way to the bottom right
                if land[i][j] == 1 && !visited[i][j] {
                    let left: usize = j;
                    let mut right: usize = j;

                    let top: usize = i;
                    let mut bottom: usize = i;

                    while right < n && land[top][right] == 1 && !visited[top][right] {
                        right += 1;
                    }

                    while bottom < m && land[bottom][left] == 1 && !visited[bottom][left] {
                        bottom += 1;
                    }

                    for row in top..bottom {
                        for col in left..right {
                            visited[row][col] = true;
                        }
                    }

                    res.push(vec![top as i32, left as i32, (bottom - 1) as i32, (right - 1) as i32]);
                    
                }
            }
        }

        res
    }
}