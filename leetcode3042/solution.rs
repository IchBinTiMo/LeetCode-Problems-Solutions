/*
Solution: Brute Force

Time: O(n^2) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.25 MB | 40.00%

- n: length of 'words'
*/

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let n: usize = words.len();

        let mut res: i32 = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                if Self::is_prefix_and_suffix(words[i].as_str(), words[j].as_str()) {
                    res += 1;
                }
            }
        }

        res
    }

    fn is_prefix_and_suffix(str1: &str, str2: &str) -> bool {
        if str1.len() > str2.len() {
            return false;
        }

        let m: usize = str1.len();
        let n: usize = str2.len();

        str1[0..m] == str2[0..m] && str1[0..m] == str2[(n - m)..n]
    }
}