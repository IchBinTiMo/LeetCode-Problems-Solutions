impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {

        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                return nums1[i];
            } else if nums1[i] < nums2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }

        -1
    }
}

// use std::collections::HashSet;

// impl Solution {
//     pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {

//         let set1: HashSet<i32> = nums1.iter().map(|&val| val).collect();
//         let set2: HashSet<i32> = nums2.iter().map(|&val| val).collect();

//         if set1.is_disjoint(&set2) {
//             return -1;
//         }

//         for &num in nums1.iter() {
//             if nums2.contains(&num) {
//                 return num;
//             }
//         }

//         -1
//     }
// }