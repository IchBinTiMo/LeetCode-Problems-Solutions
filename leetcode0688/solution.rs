impl Solution {
    pub fn knight_probability(n: i32, mut k: i32, row: i32, column: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }

        let moves: [(i32, i32); 8] = [
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (-1, 2),
            (1, 2),
            (1, -2),
            (2, 1),
            (2, -1),
        ];

        let mut dp: Vec<Vec<f64>> = vec![vec![0.0; n as usize]; n as usize];
        let mut prev: Vec<Vec<f64>> = vec![vec![0.0; n as usize]; n as usize];

        prev[row as usize][column as usize] = 1.0;

        loop {
            for i in 0..n {
                for j in 0..n {
                    for &(r, c) in moves.iter() {
                        let new_r = i + r;
                        let new_c = j + c;


                        if new_r >= 0 &&
                            new_r < n &&
                            new_c >= 0 &&
                            new_c < n {
                                dp[new_r as usize][new_c as usize] += prev[i as usize][j as usize] / 8.0;
                        }
                    }
                }
            }

            for i in 0..n {
                for j in 0..n {
                    prev[i as usize][j as usize] = dp[i as usize][j as usize];
                }
            }

            k -= 1;

            if k <= 0 {
                break;
            }
            dp = vec![vec![0.0; n as usize]; n as usize];
        }


        prev.iter().map(|row| row.iter().sum::<f64>()).sum::<f64>()
    }
}