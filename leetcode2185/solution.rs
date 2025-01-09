/*
Solution: 

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.38 MB | 44.44%

- n: length of 'words'
*/

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().fold(0, |acc, s| acc + if s.as_str().starts_with(pref.as_str()) {1} else {0})
    }
}