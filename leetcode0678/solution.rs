impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut left: i32 = 0;
        let mut right: i32 = 0;

        for c in s.chars() {
            if c == '(' {
                left += 1;
            } else {
                left -= 1;
            }

            if c != ')' {
                right += 1;
            } else {
                right -= 1;
            }

            // return false if number of '(' plus number of '*' is less than number of ')'
            if right < 0 {
                return false;
            }

            // if "left" is negative and "right" is positive, there must be an '*' counted as "right" should have been '('
            // however, if "left" is negative and "right" is 0, there must be an '*' counted as "right" should have been ''
            left = left.max(0);
        }

        left == 0
    }
}