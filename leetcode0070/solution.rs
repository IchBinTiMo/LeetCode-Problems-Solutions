impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut prev = 1;
        let mut current = 1;

        for i in 2..=n {
            let tmp = current;
            current = prev + current;
            prev = tmp;
        }

        current
    }
}