impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        /// Two Pointers
        /// 
        /// Time: O(n) | Space: O(n)
        /// where n is the length of s
        ///

        // count the differences between the chars of s and t
        let diffs: Vec<i32> = s.chars().zip(t.chars()).map(|(cs, ct)| ((cs as i32) - (ct as i32)).abs()).collect::<Vec<i32>>();

        let n: usize = diffs.len();

        let mut left: usize = 0;
        let mut right: usize = 0;

        let mut acc: i32 = 0;

        let mut res: i32 = 0;

        for i in 0..n {
            while left < right && max_cost - acc < diffs[i] {
                // shrink the window if there is no more room for increasing the cost
                acc -= diffs[left];
                acc = acc.max(0);
                left += 1;
            }

            if max_cost - acc >= diffs[i] {
                // expand the window
                acc += diffs[i];
                right += 1;

                res = res.max((right - left) as i32);
            }
        }

        res
    }
}