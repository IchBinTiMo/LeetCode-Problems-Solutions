/*
Solutoin: Brute Force

Time: O(m + n) | Space: O(m)

Runtime: 60 ms | 100.00%
Memory: 3.60 MB | 25.00%

- m: length of arr1
- n: length of arr2
*/

use std::collections::HashSet;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut prefix1: HashSet<i32> = HashSet::new();

        for &num in arr1.iter() {
            let mut num: i32 = num;

            while num > 0 {
                prefix1.insert(num);
                num /= 10;
            }
        }

        let mut res: u32 = u32::MIN;

        for &num in arr2.iter() {
            let mut num: i32 = num;

            while num > 0 {
                if prefix1.contains(&num) {
                    res = res.max(num.ilog10() + 1);
                    break;
                }
                num /= 10;
            }
        }

        res as i32
    }
}