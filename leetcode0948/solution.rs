impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        tokens.sort_unstable();

        let n: usize = tokens.len();

        let mut left: usize = 0;
        let mut right: usize = n - 1;

        let mut res: i32 = 0;
        let mut current: i32 = 0;

        while left <= right && left < n && right < n {
            if tokens[left] > power {
                if current < 1 {
                    break;
                }
                current -= 1;
                power += tokens[right];
                right -= 1;
            } else {
                current += 1;
                power -= tokens[left];
                left += 1;
            }

            res = res.max(current);
        }

        res
    }
}