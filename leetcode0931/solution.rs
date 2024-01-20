impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len() - 1;
        for r in 1..=n {
            for c in 0..=n {
                if c == 0 {
                    matrix[r][c] += matrix[r - 1][c].min(matrix[r - 1][c + 1]);
                } else if c == n {
                    matrix[r][c] += matrix[r - 1][c].min(matrix[r - 1][c - 1]);
                } else {
                    matrix[r][c] += matrix[r - 1][c - 1].min(matrix[r - 1][c].min(matrix[r - 1][c + 1]));
                }
            }
        }

        *matrix[n].iter().min().unwrap()
    }
}