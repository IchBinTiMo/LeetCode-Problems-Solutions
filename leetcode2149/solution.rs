impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; nums.len()];

        let (mut pos, mut neg) = (0, 1);

        for &num in nums.iter() {
            match num > 0 {
                true => {
                    ans[pos] = num;
                    pos += 2;
                },
                _ => {
                    ans[neg] = num;
                    neg += 2;
                }
            }
        }
        ans
    }
}

// impl Solution {
//     pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
//         let mut positive: Vec<i32> = Vec::new();
//         let mut negative: Vec<i32> = Vec::new();

//         for &num in nums.iter() {
//             match num > 0 {
//                 true => positive.push(num),
//                 _ => negative.push(num)
//             }
//         }

//         let mut ans: Vec<i32> = Vec::new();

//         for i in 0..nums.len() / 2 {
//             ans.push(positive[i]);
//             ans.push(negative[i]);
//         }

//         ans
//     }
// }