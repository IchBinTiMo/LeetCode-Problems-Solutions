/*
Solution: String

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.22 MB | 61.11%

- n: length of 's'
*/

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut chars: Vec<char> = Vec::new();

        for ch in s.chars() {
            if let Some(&c) = chars.last() {
                if (ch as u8) - b'0' <= 9 && (c as u8) - b'0' > 9 {
                    chars.pop();
                } else {
                    chars.push(ch);
                }
            } else {
                chars.push(ch);
            }
        }

        chars.iter().collect::<String>()
    }
}