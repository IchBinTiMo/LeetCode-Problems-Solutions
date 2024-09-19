/*
Solution: Recursion

Time: O(3 ^ n) | Space: O(3 ^ n)

Runtime: 0 ms | 100.00%
Memory: 2.08 MB | 100.00%

- n: length of 'expression'
*/

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        for (i, op) in expression.chars().enumerate() {
            if op == '+' || op == '-' || op == '*' {
                let r1: Vec<i32> = Self::diff_ways_to_compute(String::from(&expression[..i]));
                let r2: Vec<i32> = Self::diff_ways_to_compute(String::from(&expression[(i + 1)..]));

                for &m in r1.iter() {
                    for &n in r2.iter() {
                        match op {
                            '+' => res.push(m + n),
                            '-' => res.push(m - n),
                            '*' => res.push(m * n),
                            _ => {}
                        }
                    }
                }
            }
        }

        if res.is_empty() {
            res.push(expression.parse().unwrap());
        }

        res
    }
}