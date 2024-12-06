/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 17 ms | 100.00%
Memory: 2.37 MB | 75.00%

*/

use std::collections::HashSet;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned: HashSet<i32> = HashSet::from_iter(banned.into_iter());

        let mut acc: i32 = 0;

        let mut res: i32 = 0;

        let mut i: i32 = 1;

        while i <= n && acc + i <= max_sum {
            if !banned.contains(&i) {
                acc += i;
                res += 1;
            }
            i += 1;
        }

        res
    }
}