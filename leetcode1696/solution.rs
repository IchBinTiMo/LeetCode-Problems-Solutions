use std::collections::VecDeque;

impl Solution {
    pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
        let n: usize = nums.len();

        let mut queue: VecDeque<usize> = VecDeque::new();

        queue.push_back(0);

        for i in 1..n {
            while let Some(idx) = queue.front() {
                if i - idx > k as usize {
                    queue.pop_front();
                } else {
                    break;
                }
            }

            nums[i] += nums[*queue.front().unwrap()];

            while let Some(&idx) = queue.back() {
                if nums[i] > nums[idx] {
                    queue.pop_back();
                } else {
                    break;
                }
            }

            queue.push_back(i);
        }

        *nums.last().unwrap()
    }
}