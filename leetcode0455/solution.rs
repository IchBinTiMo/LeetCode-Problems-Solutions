impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {

        g.sort_unstable();
        s.sort_unstable();

        let mut ans = 0;

        while let Some(size) = s.pop() {
            while let Some(greed) = g.pop() {
                if greed <= size {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}