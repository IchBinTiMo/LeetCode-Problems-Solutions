/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.06 MB | 92.68%

- n: length of 's'
*/

impl Solution {
    pub fn get_lucky(s: String, mut k: i32) -> i32 {
        let mut res: i32 = 0;

        for byte in s.bytes() {
            let mut byte: i32 = (byte - 96) as i32;

            while byte > 0 {
                res += byte % 10;
                byte /= 10;
            }
        }

        while k > 1 {
            let mut current: i32 = res;

            res = 0;

            while current > 0 {
                res += current % 10;
                current /= 10;
            }
            
            current = res;
            k -= 1;
        }

        res
    }
}
