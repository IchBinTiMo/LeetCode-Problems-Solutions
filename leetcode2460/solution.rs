/*
Solution: Two Pointers

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.37 MB | 72.73%

- n: length of 'nums'
*/

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();

        let mut arr: Vec<i32> = (0..n).fold(Vec::new(), |mut v, i| {
            if i == n - 1 {
                v.push(nums[i]);
            } else {
                if nums[i] == nums[i + 1] {
                    v.push(nums[i] * 2);
                    nums[i + 1] = 0;
                } else {
                    v.push(nums[i]);
                }
            }
            v
        });

        let mut left: usize = 0;
        let mut right: usize = 1;

        while left < n - 1 && right < n {
            if arr[left] != 0 {
                left += 1;
                continue;
            } else {
                right = left + 1;

                while right < n {
                    if arr[right] == 0 {
                        right += 1;
                        continue;
                    } else {
                        arr[left] = arr[right];
                        arr[right] = 0;
                        break;
                    }
                }

                left += 1;
            }
        }


        arr
    }
}