impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![vec![0;  matrix.len()]; matrix[0].len()];

        for j in 0..ans.len() {
            for i in 0..ans[0].len() {
                ans[j][i] = matrix[i][j];
            }
        }

        ans
    }
}