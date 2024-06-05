impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        /// Brute Force
        /// 
        /// Time: O(n ^ 2) | Space: O(1)
        /// where n is the sum of length of all words
        
        // array to store the common frequency of each letter
        let mut common: [i32; 26] = [i32::MAX; 26];

        for word in words.iter() {
            // array to store the frequency of each letter
            let mut freqs: [i32; 26] = [0; 26];

            for byte in word.bytes() {
                freqs[(byte as usize) - 97] += 1;
            }

            // update the common frequency
            for i in 0..26 {
                common[i] = common[i].min(freqs[i]);
            }
        }

        let mut res: Vec<String> = Vec::new();

        // push the common letters
        for i in 0..26 {
            for _ in 0..common[i] {
                res.push((((i + 97) as u8) as char).to_string());
            }
        }

        res
    }
}