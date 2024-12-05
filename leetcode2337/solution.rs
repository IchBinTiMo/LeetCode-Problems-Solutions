/*
Solution: Two Pointers

Time: O(n) | Space: O(n)

Runtime: 8 ms | 100.00%
Memory: 4.78 MB | 100.00%

- n: length of 'start'
*/

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let mut v: Vec<(char, usize)> = Vec::new();

        for (i, c) in start.chars().enumerate() {
            if c != '_' {
                v.push((c, i));
            }
        }

        for (i, c) in target.chars().rev().enumerate() {
            let i: usize = target.len() - i - 1;
            if c == '_' {
                continue;
            } else {
                if let Some((current, idx)) = v.pop() {
                    if (c == 'R' && c == current && idx <= i) || 
                        (c == 'L' && c == current && idx >= i) {
                            continue;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        v.is_empty()
    }
}