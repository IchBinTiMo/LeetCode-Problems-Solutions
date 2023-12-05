impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut ans = 0;

        while n > 1 {
            match n % 2 {
                0 => {
                    ans += n / 2;
                    n /= 2;
                },
                _ => {
                    ans += n / 2;
                    n = n / 2 + 1;
                }
            }
        }

        ans
    }
}