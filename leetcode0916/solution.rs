/*
Solution: 

Time: O(m + n) | Space: O(1)

Runtime: 3 ms | 100.00%
Memory: 3.52 MB | 75.00%

- m: length of 'words1'
- n: length of 'words2'
*/

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        let mut freqs_2: [i32; 26] = [0; 26];

        for w in words2 {
            let mut freq: [i32; 26] = [0; 26];

            for b in w.as_bytes() {
                freq[(b - b'a') as usize] += 1;
            }

            for i in 0..26 {
                freqs_2[i] = freqs_2[i].max(freq[i]);
            }
        }

        for w in words1 {
            let mut freq: [i32; 26] = [0; 26];

            for b in w.as_bytes() {
                freq[(b - b'a') as usize] += 1;
            }

            let mut valid: bool = true;

            for i in 0..26 {
                if freq[i] < freqs_2[i] {
                    valid = false;
                    break;
                }
            }

            if valid {
                res.push(w.to_string());
            }
        }

        res
    }
}