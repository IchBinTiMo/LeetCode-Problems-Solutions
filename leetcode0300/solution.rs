use std::cmp::Ordering;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        let mut queue: Vec<i32> = Vec::new();

        queue.push(nums[0]);

        for i in 1..nums.len() {
            match nums[i].cmp(queue.last().unwrap()) {
                Ordering::Equal => continue,
                Ordering::Greater => queue.push(nums[i]),
                Ordering::Less => {
                    match queue.binary_search(&nums[i]) {
                        Ok(_) => {},
                        Err(idx) => queue[idx] = nums[i]
                    }
                }
            }
        }

        queue.len() as i32
    }
}

// impl Solution {
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         if nums.len() == 1 {
//             return 1;
//         }

//         let mut dp: Vec<i32> = vec![1; nums.len()];

//         for i in 0..nums.len() {
//             for j in 0..i {
//                 if nums[j] < nums[i] {
//                     dp[i] = dp[i].max(dp[j] + 1);
//                 }
//             }
//         }

//         *dp.iter().max().unwrap()
//     }
// }