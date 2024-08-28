/*
Solution: DFS

Time: O(m * n) | Space: O(m * n)

Runtime: 36 ms | 85.71%
Memory: 12.53 MB | 50.00%

- m is the number of rows in grid1
- n is the number of columns in grid1
*/

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid1.len();
        let n: usize = grid1[0].len();
        let mut res: i32 = 0;

        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                let mut flag = true;
                if !visited[i][j] && grid2[i][j] == 1 {
                    Self::dfs(&mut flag, i, j, &mut visited, &grid1, &grid2);
                    if flag {
                        res += 1;
                    }
                }
            }
        }

        res
    }

    fn dfs(flag: &mut bool, r: usize, c: usize, visited: &mut Vec<Vec<bool>>, grid1: &Vec<Vec<i32>>, grid2: &Vec<Vec<i32>>) {
        if grid1[r][c] == 0 {
            *flag = false;
        }
        visited[r][c] = true;
        let dir: [i32; 5] = [0, 1, 0, -1, 0];
        let r: i32 = r as i32;
        let c: i32 = c as i32;

        for i in 0..4 {
            let new_r: usize = (r + dir[i]) as usize;
            let new_c: usize = (c + dir[i + 1]) as usize;

            if new_r >= 0 && new_r < grid2.len() 
                && new_c >= 0 && new_c < grid2[0].len() 
                && !visited[new_r][new_c] 
                && grid2[new_r][new_c] == 1 {
                    Self::dfs(flag, new_r, new_c, visited, grid1, grid2);
                }
        }
    }
}