/*
Solution: Sliding Window

Time: O(n log n) | Space: O(1)

Runtime: 18 ms | 100.00%
Memory: 3.47 MB | 100.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut left: usize = 0;
        let mut right: usize = 0;

        let mut res: usize = 0;

        while right < nums.len() {
            if nums[right] - nums[left] <= 2 * k {
                res = res.max(right - left + 1);
                right += 1;
            } else {
                while left < right && nums[right] - nums[left] > 2 * k {
                    left += 1;
                }
            }
        }

        res as i32
    }
}