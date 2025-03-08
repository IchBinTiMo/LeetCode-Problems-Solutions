/*
Solution: 

Time: O(n ^ 2) | Space: O(n ^ 2)

Runtime: 0 ms | 100.00%
Memory: 2.30 MB | 98.35%

- n: length of 'grid'
*/

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n: usize = grid.len();

        let mut visited: Vec<bool> = vec![false; 1 + n * n];

        let mut duplicate: i32 = -1;

        let mut sum: i32 = (1..=(n * n)).fold(0, |acc, x| acc + x) as i32;

        for row in grid {
            for num in row {
                if visited[num as usize] {
                    duplicate = num;
                }

                visited[num as usize] = true;

                sum -= num;
            }
        }

        vec![duplicate, duplicate + sum]
    }
}