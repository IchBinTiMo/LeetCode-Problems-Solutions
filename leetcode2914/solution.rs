/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.71 MB | 16.67%

- n: length of 's'
*/

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut res: i32 = 0;

        let chars: Vec<char> = s.chars().collect::<Vec<char>>();

        let mut i: usize = 0;

        while i < chars.len() {
            if chars[i] != chars[i + 1] {
                res += 1;
            }

            i += 2;
        }

        res
    }
}