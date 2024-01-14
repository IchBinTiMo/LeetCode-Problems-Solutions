use std::collections::HashSet;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut cnt_by_chars1: [i32; 26] = [0; 26];
        let mut cnt_by_chars2: [i32; 26] = [0; 26];

        let mut visited1: HashSet<u8> = HashSet::new();
        let mut visited2: HashSet<u8> = HashSet::new();

        for byte in word1.bytes() {
            cnt_by_chars1[(byte - b'a') as usize] += 1;
            visited1.insert(byte);
        }
        
        for byte in word2.bytes() {
            cnt_by_chars2[(byte - b'a') as usize] += 1;
            visited2.insert(byte);
        }

        let mut cnt_by_cnt1: [i32; 100001] = [0; 100001];
        let mut cnt_by_cnt2: [i32; 100001] = [0; 100001];

        for &cnt in cnt_by_chars1.iter() {
            cnt_by_cnt1[cnt as usize] += 1;
        }

        for &cnt in cnt_by_chars2.iter() {
            cnt_by_cnt2[cnt as usize] += 1;
        }

        cnt_by_cnt1 == cnt_by_cnt2 && visited1 == visited2


    }
}