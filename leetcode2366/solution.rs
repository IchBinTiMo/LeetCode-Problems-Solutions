impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;

        let mut current = *nums.last().unwrap();

        for &num in nums.iter().rev() {
            if current >= num {
                current = num;
                continue;
            }

            let mut step: i64 = match num % current {
                0 => (num / current - 1) as i64,
                _ => (num / current) as i64
            };

            current = (num as i64 / (step + 1)) as i32;

            ans += step;
        }

        ans
    }
}

// impl Solution {
//     pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
//         // let mut length = (nums.len() - 1) as i32;
//         let mut step:i64 = 0;
//         let mut last:i64 = nums[nums.len() - 1] as i64;

//         for num in nums.iter().rev() {
//             if last >= (*num as i64) {
//                 last = (*num as i64);
//                 continue;
//             }

//             let mut tmp:i64 = ((*num as i64) / last) as i64;

//             if ((*num as i64) % last) > 0 {
//                 tmp += 1;
//             }

//             last = ((*num as i64) / tmp) as i64;

//             step += tmp-1;
//         } 
//         step as i64
//     }
// }