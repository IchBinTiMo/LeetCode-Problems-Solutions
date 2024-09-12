/*
Solution:

Time: O(m * n) | Space: O(1)

Runtime: 11 ms | 86.21%
Memory: 2.64 MB | 55.17%

- m: length of 'words'
- n: length of elements in 'words'
*/

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut res: i32 = 0;

        let mut mask: i32 = 0;

        for c in allowed.chars() {
            mask |= 1 << (c as u8 - ('a') as u8);
        }

        for word in words.iter() {
            let mut valid: bool = true;

            for c in word.chars() {
                if mask | 1 << (c as u8 - ('a') as u8) != mask {
                    valid = false;
                    break;
                }
            }

            if valid {
                res += 1;
            }
        }

        res
    }
}