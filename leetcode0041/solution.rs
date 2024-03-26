impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        /// Key: The missing positive must be in the range [1, n] or equal to n + 1.
        let n: usize = nums.len();

        for i in 0..n {
            // set all numbers not in the range [1, n] to n + 1
            if nums[i] <= 0 || nums[i] > n as i32 {
                nums[i] = (n + 1) as i32;
            }
        }

        // if a number x occurs, set the number at index (x - 1) to negative
        for i in 0..n {
            let num = nums[i].abs();

            if num > n as i32 {
                continue;
            }

            if nums[(num - 1) as usize] > 0 {
                nums[(num - 1) as usize] *= -1;
            }
        }

        // if the ith element is positive, i+1 is the missing positive
        for i in 0..n {
            if nums[i] >= 0 {
                return (i + 1) as i32;
            }
        }

        // if all elements are negative, n + 1 is the missing positive
        (n + 1) as i32
    }
}