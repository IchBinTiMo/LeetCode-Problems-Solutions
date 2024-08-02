/*
Solution 1:

Sliding Window

Time: O(n) | Space: O(1)

- n: length of nums
*/

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();

        let mut ones: i32 = 0;

        for &num in nums.iter() {
            ones += num;
        }

        let mut sum: i32 = 0;

        for i in 0..ones as usize {
            sum += nums[i];
        }

        let mut res: i32 = ones - sum;

        for i in 0..n {
            sum += nums[(i + ones as usize) % n] - nums[i];

            res = res.min(ones - sum);
        }

        res
    }
}