/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.22 MB | 69.23%

- n: length of 's1'
*/

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut cnt: i32 = 0;

        let mut freq: i32 = 0;

        for (b1, b2) in s1.bytes().zip(s2.bytes()) {
            if b1 != b2 {
                cnt += 1;
            }

            freq += 1 << (b1 - b'a');
            freq -= 1 << (b2 - b'a');
        }

        (cnt == 0 || cnt == 2) && freq == 0
    }
}