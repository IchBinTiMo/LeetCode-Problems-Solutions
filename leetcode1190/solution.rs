impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        /// Stack
        /// 
        /// Time: O(n * k) | Space: O(n)
        /// 
        /// where n is the length of s
        /// and k is the number of '(' in s
        let mut res: Vec<char> = Vec::new();
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if c != ')' {
                res.push(c);
            } else {
                while let Some(to_rev) = res.pop() {
                    if to_rev == '(' {
                        break;
                    }
                    stack.push(to_rev);
                }

                res.append(&mut stack);
                stack.clear();
            }


        }

        res.iter().collect::<String>()
    }
}