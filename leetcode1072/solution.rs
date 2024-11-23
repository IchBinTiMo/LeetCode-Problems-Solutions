/*
Solution: 

Time: O(m * n) | Space: O(0)

Runtime: 3 ms | 100.00%
Memory: 2.80 MB | 50.00%

- m: length of 'matrix'
- n: length of 'matrix[0]'
*/

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let m: usize = matrix.len(); 
        let n: usize = matrix[0].len(); 

        let mut sums: Vec<i32> = Vec::new();
        let mut visited: Vec<bool> = vec![false; m];

        let mut res: i32 = 0;
        

        for i in 0..m {
            let mut current: i32 = 1;

            for j in (i + 1)..m {
                if visited[j] {
                    continue;
                }

                if matrix[i] == matrix[j] {
                    current += 1;
                    visited[j] = true;
                } else {
                    let mut valid: bool = true;
                    for k in 0..n {
                        if matrix[i][k] == matrix[j][k] ^ 0 {
                            valid = false;
                            break;
                        }
                    }

                    if valid {
                        current += 1;
                        visited[j] = true;
                    }
                }
            }

            res = res.max(current);

        }

        res
    }
}