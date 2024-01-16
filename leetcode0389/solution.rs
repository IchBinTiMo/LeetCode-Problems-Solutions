impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let byte_s: u8 = s.bytes().sum();
        let byte_t: u8 = t.bytes().sum();

        char::from(byte_t - byte_s)
    }
}