impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut path: Vec<i32> = vec![];
        Solution::backtrack(0 as usize, &mut path, &mut ans, nums);
        ans
    }

    pub fn backtrack(current: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, nums: Vec<i32>) {
        if current == nums.len() {
            ans.push(path.to_vec());
            return;
        } else {
            for i in 0..nums.len() {
                if path.contains(&nums[i]) {
                    continue;
                }
                path.push(nums[i]);
                Solution::backtrack(current + 1, path, ans, nums.clone());
                path.pop();
            }
        }
    }
}