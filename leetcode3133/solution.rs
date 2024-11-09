/*
Solution: Bit Manipulation

Time: O(log n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.16 MB | 100.00%
 */

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut n: i64 = n as i64;

        let mut shift: i32 = 0;

        let mut res: i64 = x as i64;

        n -= 1;

        while n > 0 {
            if res & (1 << shift) == 0 {
                res |= ((n & 1) << shift) as i64;
                n >>= 1;
            }

            shift += 1;
        }

        res
    }
}