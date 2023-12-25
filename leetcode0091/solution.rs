use std::collections::VecDeque;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if &s[0..1] == "0" {
            return 0;
        }

        if s.len() == 1 {
            return 1;
        }

        let mut ans = 1;

        let mut chars = s.chars().collect::<VecDeque<char>>();

        while chars.len() > 0 {
            let mut current_chars: VecDeque<char> = VecDeque::new();

            while let Some(c) = chars.pop_front() {
                if c == '0' {
                    if let Some(&tmp) = chars.front() {
                        if tmp == '0' {
                            return 0;
                        }
                    }
                    if let Some(tmp) = current_chars.pop_back() {
                        if tmp >= '3' {
                            return 0;
                        }
                    }
                    break;
                }
                current_chars.push_back(c);
            }

            let tmp_str = current_chars.iter().collect::<String>();

            if tmp_str.len() == 0 {
                continue;
            }

            let mut dp = vec![0; tmp_str.len() + 1];
            dp[0] = 1;
            dp[1] = 1;

            for i in 1..tmp_str.len() {
                if &tmp_str[i - 1..i + 1] >= "10" && &tmp_str[i - 1..i + 1] <= "26" {
                    dp[i + 1] = dp[i - 1] + dp[i];
                } else {
                    dp[i + 1] = dp[i];
                }
            }

            ans *= dp[tmp_str.len()];

        }

        ans
    }
}