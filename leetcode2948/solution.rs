/*
Solution: 

Time: O(nlogn) | Space: O(n)

Runtime: 25 ms | 100.00%
Memory: 4.75 MB | 66.67%

- n: length of 'nums'
*/

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n: usize = nums.len();

        let mut indices: Vec<usize> = (0..n).collect::<Vec<usize>>();

        indices.sort_unstable_by_key(|&idx| nums[idx]);

        let mut res: Vec<i32> = vec![0; n];

        let mut left: usize = 0;

        for right in 1..=n {
            if right == n {
                let mut sorted: Vec<usize> = indices[left..right].to_vec();
                
                sorted.sort_unstable();

                for i in left..right {
                    res[sorted[i - left]] = nums[indices[i]];
                }
            } else {
                if nums[indices[right]] - nums[indices[right - 1]] <= limit {
                    continue;
                } else {
                    let mut sorted: Vec<usize> = indices[left..right].to_vec();

                    sorted.sort_unstable();

                    for i in left..right {
                        res[sorted[i - left]] = nums[indices[i]];
                    }

                    left = right;
                }
            }
        }

        res
    }
}