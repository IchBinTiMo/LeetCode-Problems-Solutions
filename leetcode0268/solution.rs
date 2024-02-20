impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = (0..=nums.len()).fold(0, |acc, num| acc + num) as i32;

        for num in nums {
            sum -= num;
        }

        sum
    }
}