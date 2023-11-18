impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; (n + 1) as usize];
        for i in 1..=n {
            ans[i as usize] = ans[(i >> 1) as usize] + (i & 1);
        }
        ans
    }
}
// impl Solution {
//     pub fn count_bits(n: i32) -> Vec<i32> {
//         let mut ans: Vec<i32> = vec![];
//         for i in 0..=n {
//             ans.push((i).count_ones() as i32);
//         }
//         ans
//     }
// }