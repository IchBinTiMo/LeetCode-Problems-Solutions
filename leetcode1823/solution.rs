impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        /// Optimized DP
        /// 
        /// Time: O(n) | Space: O(1)
        let mut res: i32 = 0;

        for i in 2..=n {
            res = (res + k) % i;
        }

        res + 1
    }
}