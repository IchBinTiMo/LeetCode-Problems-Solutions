impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut left: i32 = 0;
        let mut right: i32 = s.len() as i32 - 1;

        let s: Vec<u8> = s.bytes().collect();

        let nums = (b'0'..b'9').collect::<Vec<u8>>();

        while left < right {
            loop {
                match s[left as usize] {
                    b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => break,
                    _ => left += 1
                }

                if left >= s.len() as i32 {
                    break;
                }
            }

            loop {
                match s[right as usize] {
                    b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => break,
                    _ => right -= 1
                }

                if right < 0 {
                    break;
                }
            }

            if left > right {
                break;
            }

            if nums.contains(&s[left as usize]) || nums.contains(&s[right as usize]) {
                match (s[left as usize] as i32 - s[right as usize] as i32).abs() {
                    0 => {},
                    _ => return false
                }
            } else {
                match (s[left as usize] as i32 - s[right as usize] as i32).abs() {
                    0 | 32 => {},
                    _ => return false
                }
            }


            left += 1;
            right -= 1;

        }
        true
    }
}