use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 1 {
            return nums;
        }

        let mut mq: VecDeque<(usize, i32)> = VecDeque::new();
        let mut ans: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            while let Some((idx, n)) = mq.pop_front() {
                if i + 1 <= k as usize || idx >= (i + 1) - (k as usize) {
                    mq.push_front((idx, n));
                    break;
                }
            }
            while let Some((idx, n)) = mq.pop_back() {
                if n > nums[i] {
                    mq.push_back((idx, n));
                    break;
                }
            }
            mq.push_back((i, nums[i]));
            if k <= (i + 1) as i32 {
                ans.push(mq[0].1);
            }

        }


        ans
    }
}