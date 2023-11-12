impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;

        for i in 0..nums.len() {
            ones = (ones ^ nums[i]) & !twos;
            twos = (twos ^ nums[i]) & !ones;
        }

        ones
    }
}