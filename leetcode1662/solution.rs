impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.iter().fold(String::from(""), |acc, s| acc + s) == word2.iter().fold(String::from(""), |acc, s| acc + s)
    }
}