impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        let mut cnt = 0;

        while n > 0 {
            cnt += (n & 1) as i32;
            n >>= 1;
        }

        cnt
    }
}
// impl Solution {
//     pub fn hammingWeight (n: u32) -> i32 {
//         n.count_ones() as i32
//     }
// }