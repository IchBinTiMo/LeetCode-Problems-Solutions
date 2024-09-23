/*
Solution: DP

Time: O(n^2 + k) | Space: O(n + k)

Runtime: 39 ms | 18.18%
Memory: 2.14 MB | 90.91%

- n: length of 's'
- k: length of 'dictionary'
*/

use std::collections::HashSet;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n: usize = s.len();

        let set: HashSet<String> = HashSet::from_iter(dictionary);

        let mut dp: Vec<usize> = vec![n; n + 1];


        for i in 1..=n {
            dp[i] = dp[i].min(dp[i - 1]);

            if set.contains(&s[0..i].to_string()) {
                dp[i] = dp[i].min(n - i);
                continue;
            } else {
                for j in 0..i {
                    if set.contains(&s[j..i].to_string()) {
                        dp[i] = dp[i].min(dp[j] - (i - j));
                    }
                }
            }
        }

        *dp.last().unwrap() as i32
    }
}