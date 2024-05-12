impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n: usize = grid.len();

        let mut res: Vec<Vec<i32>> = vec![vec![i32::MIN; n - 2]; n - 2];

        for i in 1..(n - 1) {
            for j in 1..(n - 1) {
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        res[i - 1][j - 1] = grid[((i as i32) + dy) as usize][((j as i32) + dx) as usize].max(res[i - 1][j - 1]);
                    }
                }
            }
        }

        res
    }
}