use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut idx: HashMap<i32, usize> = HashMap::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        ans.push(Vec::new());

        for &num in nums.iter() {
            if let Some(&i) = idx.get(&num) {
                if i >= ans.len() {
                    ans.push(Vec::new());
                }
                ans[i].push(num);
                idx.insert(num, i + 1);
            } else {
                ans[0].push(num);
                idx.insert(num, 1);
            }
        }

        ans
    }
}