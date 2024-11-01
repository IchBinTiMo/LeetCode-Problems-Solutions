/*
Solution: Stack

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.80 MB | 14.29%

- n: length of 's'
*/

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut res: Vec<char> = Vec::new();

        let mut prev: char = '.';
        let mut cnt: i32 = 1;

        for c in s.chars() {
            if c == prev {
                if cnt == 2 {
                    continue;
                } else {
                    cnt += 1;
                }
            } else {
                cnt = 1;
            }

            res.push(c);
            prev = c;
        }

        res.iter().collect::<String>()
    }
}