impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        /// Time: O(1) | Space: O(1)
        let dir: i32 = time / (n - 1);
        let rem: i32 = time % (n - 1);

        return if dir & 1 == 1 {
            n - rem
        } else {
            1 + rem
        }
    }
}