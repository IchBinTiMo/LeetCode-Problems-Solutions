/*
Solution: Brute Force

Time: O(n^2) | Space: O(n)

Runtime: 1583 ms | 7.69%
Memory: 183.46 MB | 53.85%

- n: sum of length of words in 'words'
*/

use std::collections::HashMap;

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let n: usize = words.len();

        let mut prefixes: HashMap<&str, Vec<usize>> = HashMap::new();

        let mut res: Vec<i32> = vec![0; n];

        for i in 0..words.len() {
            let word: &String = &words[i];

            for j in 0..word.len() {
                prefixes.entry(&word[0..=j]).and_modify(|v| v.push(i)).or_insert(vec![i]);
            }
        }

        for (k, values) in prefixes.into_iter() {
            for &v in values.iter() {
                res[v] += values.len() as i32;
            }
        }
        
        res
    }
}