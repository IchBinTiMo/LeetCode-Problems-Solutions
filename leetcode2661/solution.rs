/*
Solution: 

Time: O(m * n) | Space: O(m * n)

Runtime: 0 ms | 100.00%
Memory: 8.18 MB | 75.00%

- m: length of 'mat'
- n: length of 'mat[0]'
*/

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m: usize = mat.len();
        let n: usize = mat[0].len();

        let mut pos: Vec<(usize, usize)> = vec![(0, 0); 1 + m * n];

        let mut row_visited: Vec<usize> = vec![0; m];
        let mut col_visited: Vec<usize> = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                pos[mat[i][j] as usize] = (i, j);
            }
        }

        for i in 0..arr.len() {
            let num: usize = arr[i] as usize;
            let (r, c): (usize, usize) = pos[num];

            row_visited[r] += 1;
            col_visited[c] += 1;

            if row_visited[r] == n || col_visited[c] == m {
                return i as i32;
            }
        }

        0
    }
}