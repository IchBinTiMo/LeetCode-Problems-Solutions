/*
Solution: Two Pointers

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 3.74 MB | 100.00%

- n: length of 'arr'
*/
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n: usize = arr.len();

        let mut left: usize = 0;
        let mut right: usize = n - 1;

        while left < n - 1 && arr[left + 1] >= arr[left] {
            left += 1;
        }

        if left == n - 1 {
            return 0;
        }

        while right > 0 && arr[right - 1] <= arr[right] {
            right -= 1;
        }

        let mut res: usize = right.min(n - left - 1);

        let mut i: usize = 0;
        let mut j: usize = right;

        while i <= left && j < n {
            if arr[i] <= arr[j] {
                res = res.min(j - i - 1);
                i += 1;
            } else {
                j += 1;
            }
        }

        res as i32
    }
}