impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        // let mut length = (nums.len() - 1) as i32;
        let mut step:i64 = 0;
        let mut last:i64 = nums[nums.len() - 1] as i64;

        for num in nums.iter().rev() {
            if last >= (*num as i64) {
                last = (*num as i64);
                continue;
            }

            let mut tmp:i64 = ((*num as i64) / last) as i64;

            if ((*num as i64) % last) > 0 {
                tmp += 1;
            }

            last = ((*num as i64) / tmp) as i64;

            step += tmp-1;
            // println!("{}, {}", i, nums[i]);
            // if last >= nums[i] {
            //     last = nums[i];
            //     continue;
            // }
            // let mut tmp:i32 = (nums[i - 1] / last) as i32;

            // if (nums[i - 1] % last) > 0 {
            //     tmp += 1;
            // }

            // last = (nums[i - 1] / tmp) as i32;

            // step += tmp

        } 
        step as i64
    }
}