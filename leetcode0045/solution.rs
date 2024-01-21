impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {

        let mut ans: i32 = 0;
        let mut mx: i32 = 0;
        let mut limit: i32 = 0;

        for i in 0..(nums.len() - 1) {
            mx = mx.max(nums[i] + i as i32);
            if i as i32 >= limit {
                ans += 1;
                limit = mx;
            }
        }

        ans

    }
}

// impl Solution {
//     pub fn jump(nums: Vec<i32>) -> i32 {
//         let n: usize = nums.len();
//         let mut dp: Vec<i32> = vec![i32::MAX; n];

//         dp[0] = 0;

//         for i in 0..n {
//             for j in 1..=nums[i] {
//                 let j = j as usize;
//                 if i + j >= n {
//                     break;
//                 }
//                 dp[i + j] = dp[i + j].min(dp[i] + 1);
//             }
//         }


//         *dp.last().unwrap()

//     }
// }