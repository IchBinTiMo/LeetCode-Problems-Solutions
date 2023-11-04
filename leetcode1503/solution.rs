impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        left.into_iter().max().unwrap_or(0).max(n - right.into_iter().min().unwrap_or(n))
    }
}