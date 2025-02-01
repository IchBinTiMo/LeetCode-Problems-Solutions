/*
Solutioni:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.27 MB | 85.71%

- n: length of nums
*/

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if (nums[i - 1] & 1) ^ (nums[i] & 1) == 0 {
                return false;
            }
        }

        true
    }
}