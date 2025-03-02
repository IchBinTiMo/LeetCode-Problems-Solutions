/*
Solution: Two Pointers, HashMap

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.37 MB | 50.00%

- n: length of 'nums1' + length of 'nums2'
*/

use std::collections::HashMap;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut left: usize = 0;
        let mut right: usize = 0;

        let mut map: HashMap<i32, (i32, usize)> = HashMap::new();

        let mut res: Vec<Vec<i32>> = Vec::new();

        while left < nums1.len() || right < nums2.len() {
            let mut id: i32 = -1;
            let mut val: i32 = -1;

            if right >= nums2.len() || (left < nums1.len() && nums1[left][0] <= nums2[right][0]) {
                id = nums1[left][0];
                val = nums1[left][1];
                left += 1;
            } else {
                id = nums2[right][0];
                val = nums2[right][1];
                right += 1;
            }

            if let Some((acc, idx)) = map.get_mut(&id) {
                *acc += val;
                res[*idx][1] = *acc;
            } else {
                map.insert(id, (val, res.len()));
                res.push(vec![id, val]);
            }
        }

        res
    }
}