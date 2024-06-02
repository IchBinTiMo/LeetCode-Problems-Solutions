impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        /// Time: O(n) | Space: O(1)
        /// where n is the length of s
        let n: usize = s.len();

        for i in 0..n / 2 {
            s.swap(i, n - i - 1);
        }
    }
}