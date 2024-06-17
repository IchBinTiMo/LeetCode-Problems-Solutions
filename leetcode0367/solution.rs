impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        /// Binary search
        /// 
        /// Time: O(log num) | Space: O(1)
        if num == 1 {
            // corner case
            return true;
        }

        let num: i64 = num as i64;
        let mut left: i64 = 0;
        let mut right: i64 = num - 1;

        while left < right {
            let mid: i64 = (left + right) / 2;

            if mid * mid > num {
                right = mid;
            } else if mid * mid < num {
                left = mid + 1;
            } else {
                return true;
            }
        }

        false
    }
}