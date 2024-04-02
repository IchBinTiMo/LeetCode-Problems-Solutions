use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map_st: HashMap<u8, u8> = HashMap::new();
        let mut map_ts: HashMap<u8, u8> = HashMap::new();

        for(s_byte, t_byte) in s.bytes().zip(t.bytes()) {
            if let Some(&byte) = map_st.get(&s_byte) {
                if byte != t_byte {
                    return false;
                }
            } else if let Some(&byte) = map_ts.get(&t_byte) {
                if byte != s_byte {
                    return false;
                }
            } else {
                map_st.insert(s_byte, t_byte);
                map_ts.insert(t_byte, s_byte);
            }
        }

        true
    }
}