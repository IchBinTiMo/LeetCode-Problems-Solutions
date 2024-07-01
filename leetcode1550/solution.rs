impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        /// Time: O(n) | Space: O(1)
        let mut count: i32 = 0;
        for num in arr {
            if num & 1 == 0 {
                count = 0;
            } else {
                count += 1;
                if count > 2 {
                    return true;
                }
            }
        }

        false
    }
}