impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m: usize = matrix.len();
        let n: usize = matrix[0].len();

        // n + 2 = matrix[0].len() + 2 borders
        let mut prev: Vec<i32> = vec![0; n + 2];

        let mut res: i32 = 0;

        for i in 0..m {
            let mut dp: Vec<i32> = vec![0; n + 2];

            for j in 1..=n {
                // if current grid is 0, set height to 0 otherwise set height to prev[j] + 1
                if matrix[i][j - 1] == '0' {
                    dp[j] = 0;
                    continue;
                } else {
                    dp[j] = prev[j] + (matrix[i][j - 1] == '1') as i32;
                }
            }

            // calculate max area, same as leetcode 84: Largest Rectangle in Histogram
            let mut stack: Vec<usize> = vec![0];

            for (idx, &height) in dp.iter().enumerate() {
                let mut left: usize = *stack.last().unwrap();

                while height < dp[left] {
                    let block_height: i32 = dp[stack.pop().unwrap()];
                    left = *stack.last().unwrap();

                    let block_width: i32 = (idx - left - 1) as i32;

                    res = res.max(block_height * block_width);
                }

                stack.push(idx);
            }

            prev = dp;
        }

        res
    }
}