impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        nums.sort();
        Solution::backtrack(0, &mut path, &mut ans, nums);

        ans
    }

    pub fn backtrack(current: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, nums: Vec<i32>) {
        if current == nums.len() {
            if ans.contains(path) {
                return;
            }
            ans.push(path.to_vec());
        } else {
            Solution::backtrack(current + 1, path, ans, nums.clone());
            path.push(nums[current]);
            Solution::backtrack(current + 1, path, ans, nums.clone());
            path.pop();
        }
    }
}