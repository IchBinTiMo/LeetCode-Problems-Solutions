impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        if !s.contains('0') {
            return s;
        }
        
        let mut bytes: Vec<u8> = s.bytes().collect();

        bytes.sort_unstable_by(|a, b| b.cmp(a));

        let mut popped: u8 = 1;

        for i in 1..bytes.len() {
            if bytes[i] == 48 {
                popped = bytes.swap_remove(i - 1);
                break;
            }
        }

        bytes.push(popped);

        String::from_utf8(bytes).unwrap()
    }
}