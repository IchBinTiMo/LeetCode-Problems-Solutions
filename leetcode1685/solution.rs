impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left: i32 = 0;
        let mut right: i32 = nums.iter().sum();
        let mut result: Vec<i32> = vec![0; n];


        for i in 0..n {
            let num = nums[i];
            result[i] = ((num * (i as i32)) - left) + (right - (num * ((n - i) as i32)));
            left += num;
            right -= num;
        }

        result
    }
}