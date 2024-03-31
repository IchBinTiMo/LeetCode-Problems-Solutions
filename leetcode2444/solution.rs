impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let n: usize = nums.len();

        let mut res: i32 = 0;

        let mut min_idx: i32 = -1; // the index of the last occurrence of min_k, -1 means not found
        let mut max_idx: i32 = -1; // the index of the last occurrence of max_k, -1 means not found

        let mut out_idx: i32 = -1; // the index of the last occurrence of number out of [min_k, max_k], -1 means not found

        let mut res: i64 = 0;

        for i in 0..n {
            let num: i32 = nums[i];

            if num < min_k || num > max_k {
                out_idx = i as i32;
            }

            if num == min_k {
                min_idx = i as i32;
            }

            if num == max_k {
                max_idx = i as i32;
            }

            // the reason for substracting out_idx from smaller one among min_idx and max_idx is that
            // there might be numbers between [min_k, max_k] in (out_idx, min_idx) or (out_idx, max_idx)
            // e.g. if out_idx = 1, min_idx = 3, max_idx = 5, 
            // then the subarray which is from 2nd elements to 5th elements
            // and the subarray which is from 3rd elements to 5th elements
            // should both be counted
            res += 0.max(min_idx.min(max_idx) - out_idx) as i64;
        }

        res
    }
}