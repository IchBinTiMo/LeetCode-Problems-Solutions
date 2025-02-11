/*
Solution: Stack

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.26 MB | 100.00%

- n: length of 's'
*/

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let n: usize = part.len();
        let mut stack: String = String::new();

        for c in s.chars() {
            stack.push(c);
            if stack.len() >= n && &stack[(stack.len() - n)..stack.len()] == part.as_str() {
                for _ in 0..n {
                    stack.pop();
                }
            }

        }

        stack
    }
}