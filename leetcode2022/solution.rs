/*
Solution: 

Time: O(m * n) | Space: O(m * n)

Runtime: 26 ms | 80.00%
Memory: 4.65 MB | 13.33%
*/

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if m * n != original.len() as i32 {
            return Vec::new();
        }

        let mut res: Vec<Vec<i32>> = Vec::new();

        for i in 0..m {
            let mut row: Vec<i32> = Vec::new();

            for j in 0..n {
                row.push(original[(i * n + j) as usize]);
            }

            res.push(row);
        }

        res
    }
}