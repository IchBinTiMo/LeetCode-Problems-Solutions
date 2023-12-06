impl Solution {
    pub fn total_money(mut n: i32) -> i32 {
        let mut ans = 0;

        for i in 1..=((n + 7 - 1) / 7) {
            for j in i..(i as i32 + n.min(7)) {
                ans += j;
            }
            n -= 7;
        }

        ans
    }
}