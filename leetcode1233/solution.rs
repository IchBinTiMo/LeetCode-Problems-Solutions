/*
Solution: 

Time: O(n log n) | Space: O(n)

Runtime: 11 ms | 100.00%
Memory: 6.78 MB | 100.00%

- n: length of 'folder'
*/

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();

        let mut left: usize = 0;
        let mut right: usize = 1;

        let mut res: Vec<String> = Vec::new();

        loop {
            let n: usize = folder[left].len();

            let prefix: &str = &folder[left][0..n];

            res.push(format!("{}", folder[left]));

            while right < folder.len() && folder[right].len() >= n && &folder[right][0..n] == prefix {
                if &folder[right][n..(n + 1)] != "/" {
                    break;
                } else {
                    right += 1;
                }
            }

            left = right;
            right += 1;

            if left >= folder.len() {
                break;
            }
        }

        res
    }
}