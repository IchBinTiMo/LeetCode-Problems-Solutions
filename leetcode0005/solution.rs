impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        
        if len < 2 {
            return s;
        }
        
        let mut start = 0;
        let mut max_len = 1;
        
        for i in 0..len {
            let mut left = i as i32;
            let mut right = i as i32;
            
            // Expand around the center
            while right < len as i32 - 1 && chars[right as usize] == chars[(right + 1) as usize] {
                right += 1;
            }
            
            while left > 0 && right < len as i32 - 1 && chars[(left - 1) as usize] == chars[(right + 1) as usize] {
                left -= 1;
                right += 1;
            }
            
            let curr_len = right - left + 1;
            
            if curr_len > max_len {
                start = left as usize;
                max_len = curr_len;
            }
        }
        
        s[start..start+(max_len as usize)].to_string()
    }
}