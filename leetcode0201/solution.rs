impl Solution {
    pub fn range_bitwise_and(left: i32, mut right: i32) -> i32 {
        while right > left {
            right &= right - 1;
        }
        right
    }
}

// impl Solution {
//     pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
//         if left == 0 {
//             return 0;
//         }

//         if left != right {
//             return Solution::range_bitwise_and(left >> 1, right >> 1) << 1;
//         } else {
//             return right;
//         }
//     }
// }