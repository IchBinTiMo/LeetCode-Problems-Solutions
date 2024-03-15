impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();

        let mut forward: Vec<i32> = vec![1; n + 2];
        let mut backward: Vec<i32> = vec![1; n + 2];

        for i in 1..=n {
            forward[i] = forward[i - 1] * nums[i - 1];
        }

        for i in (1..=n).rev() {
            backward[i] = backward[i + 1] * nums[i - 1];
        }

        let mut res: Vec<i32> = Vec::new();

        for i in 1..=n {
            res.push(forward[i - 1] * backward[i + 1]);
        }

        res
    }
}