/*
Solution: 

Time: O(n) | Space: O(1)

Runtime: 1 ms | 6.77%
Memory: 2.21 MB | 48.05%
*/

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let mut res: i64 = 1;

        for i in 2..=n {
            res += 4 * (i - 1) as i64;
        }

        res
    }
}