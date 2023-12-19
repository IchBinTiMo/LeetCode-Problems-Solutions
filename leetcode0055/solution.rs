impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut current = 0;
        for i in 0..nums.len() {
            if current < (i as i32) {
                return false;
            }
            current = current.max(nums[i] + i as i32);
        }

        true
    }
}