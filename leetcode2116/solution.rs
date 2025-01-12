/*
Solution: Stack

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 3.41 MB | 20.00%

- n: length of 's'
*/

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let n: usize = s.len();

        let s: &[u8] = s.as_bytes();
        let locked: &[u8] = locked.as_bytes();

        let mut left: Vec<usize> = Vec::new();
        let mut unlocked: Vec<usize> = Vec::new();

        for i in 0..n {
            if locked[i] == b'0' {
                unlocked.push(i);
            } else if s[i] == b'(' {
                left.push(i);
            } else {
                if let Some(z) = left.pop() {
                } else if let Some(x) = unlocked.pop() {
                } else {
                    return false;
                }
            }
        }

        while let Some(i) = left.pop() {
            if let Some(idx) = unlocked.pop() {
                if i > idx {
                    return false;
                }
            } else {
                return false;
            }
        }

        left.is_empty() && unlocked.len() & 1 == 0
    }
}