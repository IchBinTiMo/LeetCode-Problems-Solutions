impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();

        let mut onesRows: Vec<i32> = vec![0; m];
        let mut zeroesRows: Vec<i32> = vec![0; m];
        let mut onesCols: Vec<i32> = vec![0; n];
        let mut zeroesCols: Vec<i32> = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    1 => {
                        onesRows[i] += 1;
                        onesCols[j] += 1;
                    },
                    _ => {
                        zeroesRows[i] += 1;
                        zeroesCols[j] += 1;
                    }
                }
            }
        }

        let mut ans: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                ans[i][j] = onesRows[i] + onesCols[j] - zeroesRows[i] - zeroesCols[j];
            }
        }

        ans
    }
}