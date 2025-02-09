/*
Solution: Hashmap

Time: O(n) | Space: O(n)

Runtime: 8 ms | 100.00%
Memory: 5.19 MB | 100.00%

- n: length of 'nums'
*/

use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n: usize = nums.len();

        let mut ht: HashMap<i32, i32> = HashMap::new();
        let mut cnt: i64 = 0; // number of good pairs

        for i in 0..n {
            let diff: i32 = i as i32 - nums[i];

            if let Some(c) = ht.get_mut(&diff) {
                cnt += *c as i64;
                *c += 1;
            } else {
                ht.insert(diff, 1);
            }
        }

        (n * (n - 1) / 2) as i64 - cnt
    }
}