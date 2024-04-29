impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().fold(k, |acc, &num| acc ^ num).count_ones() as i32
    }
}

// impl Solution {
//     pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
//         let mut diff: i32 = (1..nums.len()).fold(nums[0], |acc, i| acc ^ nums[i]) ^ k;

//         diff.count_ones() as i32
//     }
// }