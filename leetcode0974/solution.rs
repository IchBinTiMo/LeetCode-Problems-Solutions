use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        /// Time: O(n) | Space: O(n)
        /// where n is the length of nums
        let n: usize = nums.len();

        let mut sum: i32 = 0;
        let mut ht: HashMap<i32, i32> = HashMap::new();

        ht.insert(0, 1);

        // count the occurrence of prefix sum % k
        for i in 0..n {
            sum += nums[i];

            // since nums may include negative numbers
            // there are probably negative prefix sums
            // if a prefix sum is negative, we need to keep adding k
            // until it becomes positive
            ht.entry(((sum % k) + k) % k).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        let mut res: i32 = 0;

        for &value in ht.values() {
            res += value * (value - 1) / 2;
        }

        res
    }
}

// use std::collections::HashMap;

// impl Solution {
//     pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
//         /// Time: O(n) | Space: O(n)
//         /// where n is the length of nums
//         let n: usize = nums.len();

//         let mut sum: i32 = 0;
//         let mut ht: HashMap<i32, i32> = HashMap::new();

//         ht.insert(0, 1);

//         // count the occurrence of prefix sum % k
//         for i in 0..n {
//             sum += nums[i];

//             ht.entry((sum % k)).and_modify(|cnt| *cnt += 1).or_insert(1);
//         }

//         let mut res: i32 = 0;

//         for &key in ht.keys() {
//             let value = *ht.get(&key).unwrap();

//             if key > 0 {
//                 // since nums may include negative numbers
//                 // we need to consider if key - k is in the hashmap
//                 if let Some(&cnt) = ht.get(&(key - k)) {
//                     res += cnt * value;
//                 }
//             }

//             res += value * (value - 1) / 2;
//         }

//         res
//     }
// }