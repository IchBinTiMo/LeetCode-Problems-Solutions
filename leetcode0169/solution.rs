impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        nums.sort_unstable();

        let mut count: i32 = 1;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                count += 1;
            } else {
                count = 1;
            }

            if count > (nums.len() as i32) / 2 {
                return nums[i];
            }
        }

        -1
    }
}