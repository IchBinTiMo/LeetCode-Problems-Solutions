use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m: usize = matrix.len();
        let n: usize = matrix[0].len();

        let mut prefix:  Vec<Vec<i32>> = vec![vec![0; n]; m];

        for i in 0..m {
            let mut tmp: i32 = 0;
            for j in 0..n {
                tmp += matrix[i][j];
                prefix[i][j] = tmp;
            }
        }

        let mut ans: i32 = 0;

        for left in 0..n {
            for right in left..n {
                let mut map: HashMap<i32, i32> = HashMap::new();
                map.insert(0, 1);

                let mut sum: i32 = 0;

                for i in 0..m {
                    sum += prefix[i][right] - prefix[i].get(left - 1).unwrap_or(&0);

                    ans += map.get(&(sum - target)).unwrap_or(&0);

                    map.insert(sum, map.get(&sum).unwrap_or(&0) + 1);
                }
            }
        }

        ans
    }
}