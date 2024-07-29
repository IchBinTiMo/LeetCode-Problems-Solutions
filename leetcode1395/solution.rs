/*
Solution 1"

Nested loops

Time: O( n ^ 2 ) | Space: O(1)

- n: length of rating
*/

use std::collections::HashMap;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let n: usize = rating.len();

        let mut res: i32 = 0;

        for i in 0..n {
            let mut bigger: (i32, i32) = (0, 0);
            let mut smaller: (i32, i32) = (0, 0);

            for j in 0..i {
                if rating[i] < rating[j] {
                    bigger.0 += 1;
                } else {
                    smaller.0 += 1;
                }
            }

            for j in (i + 1)..n {
                if rating[i] < rating[j] {
                    bigger.1 += 1;
                } else {
                    smaller.1 += 1;
                }
            }

            res += bigger.0 * smaller.1;
            res += bigger.1 * smaller.0;
        }

        res
    }
}