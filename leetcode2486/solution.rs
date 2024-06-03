impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        /// Two Pointers
        /// 
        /// Time: O(n) | Space: O(1)
        /// where n is the length of the longer string
        let mut left_s: usize = 0; // pointer for s
        let mut left_t: usize = 0; // pointer for t

        // find the longest prefix of t that is a subsequence of s
        while left_s < s.len() && left_t < t.len() {
            if &s[left_s..=left_s] == &t[left_t..=left_t] {
                left_t += 1;
            }
            left_s += 1;
        }

        // return the length of the remaining characters in t
        (t.len() - left_t) as i32
    }
}