impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, mut k: i32) -> i64 {
        happiness.sort_unstable();

        let mut res: i64 = 0;

        let mut round: i32 = 0;

        while k > 0 {
            let selected: i32 = happiness.pop().unwrap();

            res += (selected - round).max(0) as i64;

            round += 1;
            k -= 1;
        }

        res
    }
}