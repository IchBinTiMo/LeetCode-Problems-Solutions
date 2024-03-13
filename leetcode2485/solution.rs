impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut sum: i32 = 0;

        for i in 1..=n {
            sum += i;
        }

        let mut current_sum: i32 = sum;

        for i in (1..=n).rev() {
            if current_sum * 2 == sum + i {
                return i;
            }
            current_sum -= i;
        }

        -1
    }
}