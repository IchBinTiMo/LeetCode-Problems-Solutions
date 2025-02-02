/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.22 MB | 77.27%

- n: length of nums
*/

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n: usize = nums.len();

        let mut cnt: i32 = 0;

        for i in 0..n {
            if nums[i] > nums[(i + 1) % n] {
                cnt += 1;
            }
        }

        cnt < 2
    }
}