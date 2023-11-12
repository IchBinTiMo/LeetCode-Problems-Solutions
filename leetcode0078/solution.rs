impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        Solution::backtrack(0 as usize, &mut path, &mut ans, nums);
        ans
    }

    pub fn backtrack(current: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, nums: Vec<i32>) {
        if nums.len() == current {
            ans.push(path.to_vec());
            
        } else {
            Solution::backtrack(current + 1, path, ans, nums.clone());
            path.push(nums[current]);
            Solution::backtrack(current + 1, path, ans, nums.clone());
            path.pop();
        }

    }
}