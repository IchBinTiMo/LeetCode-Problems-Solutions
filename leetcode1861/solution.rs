/*
Solution: 

Time: O(m * n) | Space: O(m)

Runtime: 135 ms | 100.00%
Memory: 18.92 MB | 16.67%

- m: length of 'b'
- n: length of 'b[0]'
*/

impl Solution {
    pub fn rotate_the_box(mut b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m: usize = b.len();
        let n: usize = b[0].len();

        let mut res: Vec<Vec<char>> = vec![vec!['.'; m]; n];

        let mut rights: Vec<usize> = vec![n - 1; m];

        for i in 0..m {
            while let Some(ch) = b[i].pop() {
                match ch {
                    '#' => {
                        res[rights[i]][m - i - 1] = '#';
                        rights[i] -= 1;
                    },
                    '*' => {
                        res[b[i].len()][m - i - 1] = '*';
                        rights[i] = b[i].len() - 1;
                    },
                    _ => ()
                }
            }
        }

        res
    }
}