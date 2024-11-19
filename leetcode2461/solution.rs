/*
Solution: Sliding Window

Time: O(n) | Space: O(n)

Runtime: 19 ms | 89.47%
Memory: 3.84 MB | 90.48%

- n: length of 'nums'
*/

use std::collections::HashSet;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k: usize = k as usize;
        let mut set: HashSet<i32> = HashSet::new();
        let mut res: i64 = 0;
        let mut sum: i64 = 0;

        let mut left: usize = 0;

        for right in 0..nums.len() {
            let num: i32 = nums[right];

            while left < right && set.contains(&num) {
                set.remove(&nums[left]);
                sum -= nums[left] as i64;
                left += 1;
            }

            set.insert(num);
            sum += num as i64;

            if set.len() == k {
                res = res.max(sum);
                set.remove(&nums[left]);
                sum -= nums[left] as i64;
                left += 1;
            }
        }

        res

    }
}