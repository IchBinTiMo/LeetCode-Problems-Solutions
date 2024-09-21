/*
Solution: Two Pointers

Time: O(m + n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.16 MB | 49.58%
*/

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m: usize = m as usize;
        let n: usize = n as usize;

        let mut i: usize = m - 1;
        let mut j: usize = n - 1;
        let mut k: usize = m + n - 1;

        while i < m && j < n {
            if nums1[i] > nums2[j] {
                nums1[k] = nums1[i];
                i -= 1;
            } else {
                nums1[k] = nums2[j];
                j -= 1;
            }
            k -= 1;
        }

        while j < n {
            nums1[k] = nums2[j];
            j -= 1;
            k -= 1;
        }
    }
}