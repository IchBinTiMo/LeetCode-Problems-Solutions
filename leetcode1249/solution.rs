impl Solution {
    pub fn min_remove_to_make_valid(mut s: String) -> String {
        let mut stack: Vec<char> = Vec::new();

        let mut count: i32 = 0;

        while let Some(c) = s.pop() {
            if c == '(' && count >= 0 {
                continue;
            } else {
                if c == ')' {
                    count -= 1;
                } else if c == '(' {
                    count += 1;
                }

                stack.push(c);
            }
        }
        count = 0;

        while let Some(c) = stack.pop() {
            if c == ')' && count <= 0 {
                continue;
            } else {
                if c == ')' {
                    count -= 1;
                } else if c == '(' {
                    count += 1;
                }

                s.push(c);
            }
        }

        s
    }
}