/*
Solution: Stack

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.08 MB | 93.75%

- n: length of 's'
*/

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if c == 'B' {
                if let Some(&v) = stack.last() {
                    if v == 'A' {
                        stack.pop();
                        continue;
                    }
                }
            } else if c == 'D' {
                if let Some(&v) = stack.last() {
                    if v == 'C' {
                        stack.pop();
                        continue;
                    }
                }
            }
            
            stack.push(c);
        }

        stack.len() as i32
    }
}