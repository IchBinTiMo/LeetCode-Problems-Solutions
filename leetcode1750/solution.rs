impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let bytes: Vec<u8> = s.bytes().collect::<Vec<u8>>();

        let mut left: usize = 0;
        let mut right: usize = s.len() - 1;

        while left < right {
            if bytes[left] != bytes[right] {
                break;
            }

            let byte: u8 = bytes[left];

            while bytes[left] == byte {
                if left == right {
                    break;
                }
                left += 1;
            }

            while bytes[right] == byte {
                if left > right {
                    break;
                }
                right -= 1;
            }
        }

        (right - left + 1) as i32
    }
}