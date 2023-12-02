use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut chars_map: HashMap<char, i32> = HashMap::new();

        for c in chars.chars() {
            *chars_map.entry(c).or_insert(0) += 1;
        }

        let mut ans = 0;

        for word in words.iter() {
            let mut tmp = chars_map.clone();
            let mut cnt = 0;

            for c in word.chars() {
                match tmp.get(&c) {
                    Some(val) => {
                        if *val == 0 {
                            cnt = 0;
                            break;
                        } else {
                            *tmp.get_mut(&c).unwrap() -= 1;
                            cnt += 1;
                        }
                    },
                    None => {
                        cnt = 0;
                        break;
                    }
                }
            }
            ans += cnt;
        }

        ans
    }
}