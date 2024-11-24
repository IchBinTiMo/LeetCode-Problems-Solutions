/*
Solution: Greedy

Time: O(n^2) | Space: O(1)

Runtime:  ms | 100.00%
Memory: 3.20 MB | 100.00%

- n: length of 'matrix'
*/

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let n: usize = matrix.len();

        let mut res: i64 = 0;

        let mut cnt: i32 = 0;
        let mut mini: i32 = i32::MAX;

        let mut zero: bool = false;

        for i in 0..n {
            for j in 0..n {
                mini = mini.min(matrix[i][j].abs());
                
                if matrix[i][j] < 0 {
                    cnt += 1;
                }

                if matrix[i][j] == 0 {
                    zero = true;
                }

                res += (matrix[i][j].abs() as i64);
            }
        }

        if cnt & 1 == 1 && !zero {
            res - (2 * mini as i64)
        } else {
            res
        }
    }
}