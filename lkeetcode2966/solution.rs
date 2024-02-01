impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut ans: Vec<Vec<i32>> = Vec::new();

        let mut current: usize = 0;

        while current < nums.len() {

            if nums[current + 2] - nums[current] > k {
                return Vec::new();
            }

            ans.push(vec![nums[current], nums[current + 1], nums[current + 2]]);

            current += 3;
        }

        ans
    }
}

// impl Solution {
//     pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
//         nums.sort_unstable();

//         if nums.chunks(3).map(|c| c[2] - c[0]).max().unwrap() > k {
//             return Vec::new();
//         }

//         nums.chunks(3).map(|c| c.to_vec()).collect()
//     }
// }

// // impl Solution {
// //     pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
// //         nums.sort_unstable();

// //         let mut ans: Vec<Vec<i32>> = Vec::new();

// //         let mut current: usize = 0;

// //         while current < nums.len() {
// //             let mut tmp: Vec<i32> = Vec::new();

// //             if nums[current + 2] - nums[current] > k {
// //                 return Vec::new();
// //             }

// //             for i in 0..3 {
// //                 let i = i as usize;

// //                 tmp.push(nums[current + i]);
// //             }

// //             ans.push(tmp);

// //             current += 3;
// //         }

// //         ans
// //     }
// // }