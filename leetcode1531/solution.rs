impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        if s.len() == k as usize {
            return 0;
        }

        let group_count = |count: i32| match count {
            1 => 0,
            2..=9 => 1,
            10..=99 => 2,
            _ => 3
        };
            

        let s = s.as_bytes();

        let n = s.len();

        let mut dp: Vec<Vec<i32>> = vec![vec![101; k as usize + 1]; n + 1];
        
        dp[0][0] = 0;

        for i in 1..=n {
            for j in 0..=k as usize {

                let mut cnt = 0;
                let mut del = 0;

                for l in (0..i).rev() {
                    if s[i - 1] == s[l] {
                        cnt += 1;
                    } else {
                        del += 1;
                    }

                    if j >= del {
                        dp[i][j] = dp[i][j].min(dp[l][j - del] + 1 + group_count(cnt));
                    }
                }

                if j > 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                }

            }
        }

        println!("{:?}", dp);

        dp[n][k as usize]
    }
}