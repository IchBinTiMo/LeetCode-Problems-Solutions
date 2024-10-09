/*
Solution: 

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.04 MB | 76.47%

- n: length of 's'
*/

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = 0;

        for c in s.chars() {
            if c == '(' {
                left += 1;
            } else if left > 0 {
                left -= 1;
            } else {
                right += 1;
            }
        }

        left + right
    }
}