impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        ((buckets as f64).log(10 as f64) / ((minutes_to_test / minutes_to_die + 1) as f64).log(10 as f64)).ceil() as i32
    }
}