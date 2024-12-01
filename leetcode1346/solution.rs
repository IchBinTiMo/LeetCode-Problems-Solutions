/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.38 MB | 50.00%

- n: length of 'arr'
*/

use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();

        for &n in arr.iter() {
            if set.contains(&(n << 1)) {
                return true;
            } else if n & 1 == 1 {
                
            } else if set.contains(&(n >> 1)) {
                return true;
            }

            set.insert(n);
        }

        false
    }
}