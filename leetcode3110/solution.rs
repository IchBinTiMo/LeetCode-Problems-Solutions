impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        /// Time: O(N) | Space: O(N)
        let s: Vec<char> = s.chars().collect::<Vec<char>>();

        (1..s.len()).fold(0, |acc, idx| acc + ((s[idx] as i32) - (s[idx - 1] as i32)).abs())
    }
}