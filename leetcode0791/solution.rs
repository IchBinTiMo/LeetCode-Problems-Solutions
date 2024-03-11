impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        
        let mut indices: [usize; 26] = [usize::MAX; 26];

        for (idx, byte) in order.bytes().enumerate() {
            indices[(byte - b'a') as usize] = idx;
        }

        let mut bytes: Vec<u8> = s.bytes().collect();

        
        bytes.sort_unstable_by_key(|byte| indices[(byte - b'a') as usize]);

        bytes.iter().map(|&byte| char::from(byte)).collect()
    }
}