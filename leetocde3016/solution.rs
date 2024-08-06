/*
Solution 1:

Time: O(n) | Space: O(n)

- n: length of `word`
*/

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut freqs: Vec<i32> = vec![0; 26];

        for b in word.bytes() {
            freqs[b as usize - ('a' as usize)] += 1;
        }

        freqs.sort_unstable();

        let mut res: i32 = 0;
        let mut mapped: i32 = 0;

        while let Some(freq) = freqs.pop() {
            res += (mapped / 8 + 1) * freq;
            mapped += 1;
        }

        res
    }
}