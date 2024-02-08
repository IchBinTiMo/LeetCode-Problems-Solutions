impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut perfect: Vec<i32> = Vec::new();

        let mut dp: Vec<i32> = vec![0; (n + 1) as usize];

        for i in 1..=n {
            if (i as f64).sqrt() % 1.0 == 0.0 {
                perfect.push(i);
                dp[i as usize] = 1;
                continue;
            }

            let mut mn = i32::MAX;

            for &p in perfect.iter() {
                mn = mn.min(dp[p as usize] + dp[(i - p) as usize]);
                if mn == 2 {
                    break;
                }
            }

            dp[i as usize] = mn;
        }

        dp[n as usize]
    }
}