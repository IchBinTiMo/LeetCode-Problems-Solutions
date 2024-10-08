/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 4 ms | 100.00%
Memory: 4.68 MB | 100.00%

- n: length of 's'
*/

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut res: i32 = 0;
        let mut acc: i32 = 0;

        for c in s.chars() {
            match c {
                ']' => {
                    if acc <= 0 {
                        acc += 1;
                        res += 1;
                    } else {
                        acc -= 1;
                    }
                },
                '[' => acc += 1,
                _ => {}
            }
        }

        res
    }
}