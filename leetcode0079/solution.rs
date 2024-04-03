impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let mut visited: Vec<Vec<bool>> = Vec::new();

        let word: Vec<char> = word.chars().collect();

        let m: usize = board.len();
        let n: usize = board[0].len();

        for i in 0..m {
            for j in 0..n {
                if Solution::dfs(&mut board, 0, &word, i, j) {
                    return true;
                }

            }
        }

        false
    }

    fn dfs(grid: &mut Vec<Vec<char>>, current: usize, target: &[char], x: usize, y: usize) -> bool {
        if current == target.len() - 1 {
            return grid[x][y] == target[current];
        }

        let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        for &(dx, dy) in dirs.iter() {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x < 0 || new_y < 0 {
                continue;
            } else if (new_x as usize) >= grid.len() || (new_y as usize) >= grid[0].len() {
                continue;
            } else {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if grid[x][y] == target[current] {
                    grid[x][y] = '-';
                    if Solution::dfs(grid, current + 1, target, new_x, new_y) {
                        return true;
                    }
                    grid[x][y] = target[current];
                }
            }
        }

        false
    }

}