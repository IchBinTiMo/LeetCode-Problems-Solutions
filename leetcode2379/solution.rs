/*
Solution: Sliding Window

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.22 MB | 58.33%

- n: length of 'blocks'
*/

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks: &[u8] = blocks.as_bytes();
        let k: usize = k as usize;

        let mut res: i32 = i32::MAX;

        let mut left: usize = 0;

        let mut cnt: i32 = 0;

        for right in 0..blocks.len() {
            if blocks[right] == b'W' {
                cnt += 1;
            }

            if right - left == k - 1 {
                res = res.min(cnt);

                if blocks[left] == b'W' {
                    cnt -= 1;
                }

                left += 1;
            }
        }

        res
    }
}