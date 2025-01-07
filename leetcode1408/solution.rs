/*
Solution: Brute Force

Time: O(n^2) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.40 MB | 21.43%

- n: length of 'words'
*/

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let n: usize = words.len();

        let mut res: Vec<String> = Vec::new();

        for i in 0..n {
            for j in 0..n {
                if i == j || words[i].len() > words[j].len() {
                    continue;
                }

                if words[j].as_str().contains(words[i].as_str()) {
                    res.push(words[i].clone());
                    break;
                }
            }
        }
        res
    }
}