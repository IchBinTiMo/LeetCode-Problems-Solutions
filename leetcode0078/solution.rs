impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        /// Backtracking
        /// 
        /// Time: O(2^N) | Space: O(N)
        let mut res: Vec<Vec<i32>> = Vec::new();
        
        let mut path: Vec<i32> = Vec::new();
        
        Self::backtracking(&mut res, &mut path, 0, &nums);

        res
    }

    fn backtracking(res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, current: usize, nums: &Vec<i32>) {
        res.push(path.to_vec());

        for i in current..nums.len() {
            path.push(nums[i]);
            Self::backtracking(res, path, i + 1, nums);
            path.pop();
        }
    }
}

// impl Solution {
//     pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut ans: Vec<Vec<i32>> = Vec::new();
//         let mut path: Vec<i32> = Vec::new();
//         Solution::backtrack(0 as usize, &mut path, &mut ans, nums);
//         ans
//     }

//     pub fn backtrack(current: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, nums: Vec<i32>) {
//         if nums.len() == current {
//             ans.push(path.to_vec());
            
//         } else {
//             Solution::backtrack(current + 1, path, ans, nums.clone());
//             path.push(nums[current]);
//             Solution::backtrack(current + 1, path, ans, nums.clone());
//             path.pop();
//         }

//     }
// }