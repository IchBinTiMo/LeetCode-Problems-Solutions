/*
Solution: backtracking

Time: O(2^n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.10 MB | 38.46%

- n: length of 'nums'
*/

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mx: i32 = nums.iter().fold(0, |acc, &num| acc | num);

        let mut res: i32 = 0;

        Self::backtrack(&mut res, 0, mx, 0, &nums);

        res
    }

    fn backtrack(res: &mut i32, path: i32, mx: i32, current: usize, nums: &Vec<i32>) {
        if current == nums.len() {
            *res += if path == mx {1} else {0};
            return;
        }

        Self::backtrack(res, path, mx, current + 1, nums);
        Self::backtrack(res, path | nums[current], mx, current + 1, nums);
    }
}