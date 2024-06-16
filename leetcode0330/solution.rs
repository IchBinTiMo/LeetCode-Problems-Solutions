impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n : i64 = n as i64;
        let mut acc: Vec<i64> = Vec::new();

        let mut idx: usize = 0;

        let mut max: i64 = 0;

        while max < n {
            while idx < nums.len() && max + 1 >= nums[idx] as i64 {
                acc.push(nums[idx] as i64);
                max += nums[idx] as i64;
                idx += 1;
            }

            if max >= n {
                break;
            }

            acc.push(max + 1);

            max += max + 1;
        }

        for i in idx..nums.len() {
            acc.push(nums[i] as i64);
        }

        (acc.len() - nums.len()) as i32

        
    }
}