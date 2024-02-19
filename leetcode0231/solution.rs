impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}

// impl Solution {
//     pub fn is_power_of_two(n: i32) -> bool {
//         n > 0 && n.count_ones() == 1
//     }
// }