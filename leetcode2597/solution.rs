impl Solution {
    pub fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
        /// Backtracking
        /// 
        /// Time: O(2 ^ n) | Space: O(n)
        nums.sort_unstable();

        let mut res: i32 = 0;

        let mut path: [i32; 1001] = [0; 1001];

        Self::backtracking(&mut res, &mut path, 0, &nums, k);

        // return res - 1 instead of res because we need to remove the empty subset
        res - 1
    }

    fn backtracking(res: &mut i32, path: &mut [i32; 1001], current: usize, nums: &Vec<i32>, k: i32) {
        *res += 1;

        for i in current..nums.len() {
            let idx: usize = (nums[i] - k) as usize;
            // idx >= 1001 means that nums[i] - k is negative
            if idx >= 1001 || path[idx] == 0 {
                path[nums[i] as usize] += 1;
                Self::backtracking(res, path, i + 1, nums, k);
                path[nums[i] as usize] -= 1;
            }
        }
    }
}