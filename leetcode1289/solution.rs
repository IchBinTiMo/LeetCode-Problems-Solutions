impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        /// DP without optimization
        /// 
        /// Time O(n^3) | Space O(n)
        /// where n is the length of grid
        let n: usize = grid.len();

        let mut prev: Vec<i32> = grid[0].clone();

        for i in 1..n {
            let mut current: Vec<i32> = vec![i32::MAX; n];

            for j in 0..n {
                for k in 0..n {
                    if j != k {
                        current[j] = current[j].min(prev[k] + grid[i][j]);
                    }
                }
            }

            prev = current;
        }

        *prev.iter().min().unwrap()
    }
}