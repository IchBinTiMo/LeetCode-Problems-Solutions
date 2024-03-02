impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by_key(|k| k.abs());

        nums.iter().map(|e| e * e).collect::<Vec<i32>>()
    }
}

// impl Solution {
//     pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
//         nums.sort_unstable_by_key(|k| k.abs());

//         nums.iter().map(|e| e * e).collect::<Vec<i32>>()
//     }
// }