/*
Solution: Recursion

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.19 MB | 28.57%

- n: length of expression
*/

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut chars: Vec<char> = expression.chars().collect::<Vec<char>>();

        Self::recursive(&mut chars)
    }

    fn recursive(chars: &mut Vec<char>) -> bool {
        let mut stack: Vec<bool> = Vec::new();

        while let Some(c) = chars.pop() {
            match c {
                'f' => stack.push(false),
                't' => stack.push(true),
                '|' => {
                    let mut res: bool = stack.pop().unwrap();

                    while let Some(b) = stack.pop() {
                        res |= b;
                    }

                    return res;
                },
                '&' => {
                    let mut res: bool = stack.pop().unwrap();

                    while let Some(b) = stack.pop() {
                        res &= b;
                    }

                    return res;
                },
                '!' => {
                    let mut res: bool = stack.pop().unwrap();
                    
                    return !res;
                },
                ')' => {
                    stack.push(Self::recursive(chars));
                }
                _ => ()
            }
        }

        stack.pop().unwrap()
    }
}