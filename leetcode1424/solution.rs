impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        for (i, row) in nums.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if i + j == ans.len() {
                    ans.push(vec![]);
                }
                ans[i + j].insert(0, *col);
            }
        }
        ans.into_iter().flatten().collect::<Vec<i32>>()
    }
}