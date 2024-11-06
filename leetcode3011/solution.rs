/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.10 MB | 100.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
        nums.push(-1);

        let mut prev_large: i32 = i32::MIN;

        let mut curr_large: i32 = i32::MIN;
        let mut curr_small: i32 = i32::MAX;

        let mut cnt: i32 = -1;

        for &n in nums.iter() {
            if cnt == -1 {
                cnt = (n.count_ones() as i32);
            } else {
                if (n.count_ones() as i32) != cnt {
                    if prev_large > curr_small {
                        return false;
                    }

                    prev_large = curr_large;
                    curr_large = i32::MIN;
                    curr_small = i32::MAX;
                    cnt = (n.count_ones() as i32);
                }
            }

            curr_large = curr_large.max(n);
            curr_small = curr_small.min(n);
        }

        true
    }
}