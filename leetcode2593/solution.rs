/*
Solution: Sort

Time: O(n log n) | Space: O(n)

Runtime: 15 ms | 100.00%
Memory: 5.50 MB | 83.33%

- n: length of 'nums'
*/

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let n: usize = nums.len();
        let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();

        let mut marked: Vec<bool> = vec![false; n];

        nums.sort_unstable_by(|a, b| if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        });

        let mut res: i64 = 0;

        for (idx, num) in nums.into_iter() {
            if !marked[idx] {
                res += num as i64;
                marked[idx] = true;

                if idx - 1 < n {
                    marked[idx - 1] = true;
                }

                if idx + 1 < n {
                    marked[idx + 1] = true;
                }
            }
        }

        res
    }
}