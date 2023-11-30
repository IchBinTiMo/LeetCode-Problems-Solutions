impl Solution {
    pub fn minimum_one_bit_operations(mut n: i32) -> i32 {
        let mut ans: i32 = 0;
        
        while n > 0 {
            ans ^= n;
            n >>= 1;
        }
        
        ans
    }
}

// impl Solution {
//     pub fn minimum_one_bit_operations(mut n: i32) -> i32 {
//         let mut ans: i32 = 0;
//         let mut cnt = 1;
//         let mut flag = 1;

//         while n > 0 {
//             match (n & 1) {
//                 1 => {
//                     ans += ((1 << cnt) - 1) * flag;
//                     flag *= -1;
//                 },
//                 _ => {} 
//             }
//             cnt += 1;
//             n >>= 1;
//         }


//         ans.abs()
//     }
// }