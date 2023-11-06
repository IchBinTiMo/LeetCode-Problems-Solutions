impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                continue;
            } 
            ans += nums[i - 1] - nums[i] + 1;
            nums[i] = nums[i - 1] + 1;
        }
        ans
    }
}