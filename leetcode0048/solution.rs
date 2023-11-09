impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for row in 0..n {
            matrix[row].reverse();
        }

        for i in 0..(n - 1) {
            for j in 0..(n - 1 - i) {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n - 1 - j][n - 1 - i];
                matrix[n - 1 - j][n - 1 - i] = tmp;
            }
        }
    }
}