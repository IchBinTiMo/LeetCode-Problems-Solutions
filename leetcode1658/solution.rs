impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut ans: i32 = i32::MAX;

        let target: i32 = nums.iter().sum::<i32>() - x;

        let mut sum: i32 = 0;
        let mut left: usize = 0;

        for right in 0..nums.len() {
            sum += nums[right];

            while sum > target && left <= right {
                sum -= nums[left];
                left += 1;
            }

            if sum == target {
                ans = ans.min((nums.len() - (right - left + 1)) as i32);
            }
        }

        if ans > nums.len() as i32 {
            return -1;
        } else {
            return ans;
        }
    }
}