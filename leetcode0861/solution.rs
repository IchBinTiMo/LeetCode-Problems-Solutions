impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        for i in 0..m {
            if grid[i][0] == 0 {
                for j in 0..n {
                    grid[i][j] ^= 1;
                }
            }
        }

        for j in 0..n {
            let mut count: i32 = 0;

            for i in 0..m {
                count += grid[i][j];
            }

            if count <= (m as i32) / 2 {
                for i in 0..m {
                    grid[i][j] ^= 1;
                }
            }
        }

        let mut res: i32 = 0;

        for i in 0..m {
            let mut row: i32 = 0;
            for j in 0..n {
                row += grid[i][j];
                row <<= 1;
            }
            res += row;
        }

        res >> 1
    }
}