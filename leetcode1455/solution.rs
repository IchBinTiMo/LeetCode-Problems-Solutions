/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.33 MB | 29.41%

- n: length of 'sentence'
*/

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let n: usize = search_word.len();

        for (i, part) in sentence.split(" ").enumerate() {
            if part.len() >= n && &part[0..n] == &search_word[0..n] {
                return 1 + i as i32;
            }
        }
        -1
    }
}