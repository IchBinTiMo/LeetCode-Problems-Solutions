/*
Solution:

Time: O(n log(n)) | Space: O(1)

Runtime: 3 ms | 100.00%
Memory: 3.08 MB | 88.89%

- n: length of 'nums'
*/

impl Solution {
    pub fn count_ways(mut nums: Vec<i32>) -> i32 {
        nums.push(i32::MAX);

        let n: usize = nums.len();

        nums.sort_unstable();

        let mut res: i32 = if nums[0] != 0 {1} else {0};

        let mut selected: i32 = 0;

        for i in 0..(n - 1) {
            // if selecting current number would make
            // the # of selected numbers greater than current number
            // and less than the next number
            // res += 1
            if selected + 1 > nums[i] && selected + 1 < nums[i + 1] {
                res += 1;
            }

            selected += 1;
        }

        res
    }
}