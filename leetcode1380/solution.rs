impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        /// Time O(m * n) | Space O(m + n)
        /// 
        /// m -> number of rows
        /// n -> number of columns
        let mut row: Vec<i32> = vec![i32::MAX; matrix.len()];
        let mut col: Vec<i32> = vec![i32::MIN; matrix[0].len()];

        let mut res: Vec<i32> = Vec::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                row[i] = row[i].min(matrix[i][j]);
                col[j] = col[j].max(matrix[i][j]);
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if row[i] == col[j] {
                    res.push(row[i]);
                }
            }
        }

        res
    }
}