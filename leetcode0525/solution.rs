use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();

        let mut current: i32 = 0;

        let mut prefix_sum_map: HashMap<i32, usize> = HashMap::new();

        let mut res: usize = 0;

        prefix_sum_map.insert(0, 0);

        for i in 1..=n {
            current += if nums[i - 1] == 1 {1} else {-1};
            if let Some(idx) = prefix_sum_map.get(&current) {
                res = res.max(i - idx);
            } else {
                prefix_sum_map.insert(current, i);
            }
        }

        res as i32
    }
}

// use std::collections::HashMap;

// impl Solution {
//     pub fn find_max_length(nums: Vec<i32>) -> i32 {
//         let n: usize = nums.len();

//         let mut prefix_sum: Vec<i32> = vec![0; n + 1];

//         let mut prefix_sum_map: HashMap<i32, Vec<usize>> = HashMap::new();

//         prefix_sum_map.insert(0, vec![0]);

//         for i in 1..=n {
//             prefix_sum[i] = prefix_sum[i - 1] + if nums[i - 1] == 1 {1} else {-1};
//             prefix_sum_map.entry(prefix_sum[i]).and_modify(|v| v.push(i)).or_insert(vec![i]);
//         }

//         let mut res: usize = 0;

//         for v in prefix_sum_map.values() {
//             let left: usize = v[0];
//             let right: usize = v[v.len() - 1];
//             res = res.max(right - left);
//         }

//         res as i32
//     }
// }