impl Solution {
    pub fn max_depth(mut s: String) -> i32 {
        let mut res: i32 = 0;
        let mut current: i32 = 0;

        while let Some(c) = s.pop() {
            if c == ')' {
                current += 1;
                res = res.max(current);
            } else if c == '(' {
                current -= 1;
            }
        }

        res
    }
}