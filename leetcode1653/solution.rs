/*
Solution 1:

Time: O(n) | Space: O(1)

- n: length of 's'
*/

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let n: usize = s.len();

        let mut cnt_a: i32 = 0;
        let mut cnt_b: i32 = 0;

        let mut imbalance: bool = false;

        let mut res: i32 = 0;

        for c in s.chars() {
            if c == 'a' {
                if imbalance {
                    cnt_a += 1;
                }
            } else {
                if imbalance && cnt_a > cnt_b {
                    res += cnt_b;
                    cnt_a = 0;
                    cnt_b = 0;
                }

                cnt_b += 1;
                imbalance = true;
            }
        }

        if imbalance {
            res += cnt_a.min(cnt_b);
        }

        res
    }
}