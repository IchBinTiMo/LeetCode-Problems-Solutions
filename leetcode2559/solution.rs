/*
Solution: Prefix Sum

Time: O(n) | Space: O(n)

Runtime: 3 ms | 85.48%
Memory: 14.53 MB | 64.06%
*/

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n: usize = words.len();

        let mut prefix: Vec<i32> = vec![0; n + 1];

        for i in 1..=n {
            let word: &String = &words[i - 1];

            if ["a", "e", "i", "o", "u"].contains(&&word[0..1]) && 
                ["a", "e", "i", "o", "u"].contains(&&word[(word.len() - 1)..(word.len())]) {
                    prefix[i] += 1;
            }

            prefix[i] += prefix[i - 1];
        }

        let mut res: Vec<i32> = Vec::new();

        for q in queries {
            let left: usize = q[0] as usize;
            let right: usize = q[1] as usize;

            res.push(prefix[right + 1] - prefix[left]);
        }

        res
    }
}