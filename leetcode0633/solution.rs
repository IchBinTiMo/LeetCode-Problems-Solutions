impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        /// Binary search
        /// it is a simple binary search
        /// where left is 0 and right is sqrt(c)
        /// 
        /// Time O(sqrt c) | Space O(1)
        if c == 1 {
            // corner case
            return true;
        }
        let c: i64 = c as i64;

        let mut right: i64 = 0;

        while (right + 1) * (right + 1) <= c {
            right += 1;
        }

        let mut left: i64 = 0;

        while left <= right {
            if left * left + right * right > c {
                right -= 1;
            } else if left * left + right * right < c {
                left += 1;
            } else {
                return true;
            }
        }

        false
    }

    
}