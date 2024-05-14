impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut res: i32 = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 0 {
                        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

                        Self::backtracking(&mut res, grid[i][j], &mut visited, &grid, i as i32, j as i32, m, n);
                    }
            }
        }

        res
    }

    fn backtracking(ans: &mut i32, current: i32, visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<i32>>, r: i32, c: i32, m: usize, n: usize) {
        if *ans < current {
            *ans = current;
        }

        let dir: [i32; 5] = [0, 1, 0, -1, 0];

        visited[r as usize][c as usize] = true;

        for i in 0..4 {
            let new_r: usize = (r + dir[i]) as usize;
            let new_c: usize = (c + dir[i + 1]) as usize;

            if new_r < m && new_c < n && !visited[new_r][new_c] && grid[new_r][new_c] != 0 {
                Self::backtracking(ans, current + grid[new_r][new_c], visited, grid, new_r as i32, new_c as i32, m, n);
            }
        }

        visited[r as usize][c as usize] = false;
    }
}