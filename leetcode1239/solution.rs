impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {

        let bits = arr.iter().filter_map(|s| {
            let mut bitset = 0;
            for byte in s.bytes().map(|b| 1 << (b - b'a')) {
                if bitset & byte != 0 {
                    return None;
                }
                bitset |= byte;
            }

            Some(bitset)
        }).collect::<Vec<i32>>();

        let mut stack: Vec<(i32, usize)> = vec![(0, 0)];
        let mut ans = 0;

        while let Some((bitset, idx)) = stack.pop() {
            if idx == bits.len() {
                ans = ans.max(bitset.count_ones());
            } else {
                if bitset & bits[idx] == 0 {
                    stack.push((bitset | bits[idx], idx + 1));
                }
                stack.push((bitset, idx + 1));
            }
        }

        ans as i32
    }
}

// use std::collections::HashSet;

// impl Solution {
//     pub fn max_length(arr: Vec<String>) -> i32 {

//         let mut ans: i32 = 0;

//         Solution::dfs(&mut ans, arr, String::new(), 0);

//         ans
//     }

//     pub fn dfs(ans: &mut i32, arr: Vec<String>, path: String, current: usize) {
//         let mut visited: HashSet<u8> = HashSet::new();

//         for byte in path.bytes() {
//             if !visited.insert(byte) {
//                 return;
//             }
//         }

//         if *ans < path.len() as i32 {
//             *ans = path.len() as i32;
//         }

//         for i in current..arr.len() {
//             Solution::dfs(ans, arr.clone(), format!("{}{}", path, arr[i]), i + 1);
//         }

//     }
// }