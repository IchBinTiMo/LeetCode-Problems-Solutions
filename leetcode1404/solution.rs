impl Solution {
    pub fn num_steps(s: String) -> i32 {
        /// String
        /// 
        /// Time: O(n) | Space: O(n)
        let mut s: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut res: i32 = 0;

        while s != vec!['1'] {
            if *s.last().unwrap() == '1' {
                let mut left: usize = s.len() - 1; // the index of the right most zero

                while left < s.len() && s[left] != '0' {
                    left -= 1;
                }

                if left < s.len() {
                    while s.len() != left + 1 {
                        // pop all ones whose index is greater then 'left'
                        // since all of them will be popped after the "plus one" operation
                        s.pop();
                        res += 1;
                    }

                    // make the element at 'left' '1'
                    // since it will be one after the "plus one" operation
                    s[left] = '1';

                } else {
                    // if there is no '0' in the string
                    // it means all the '1' will be popped
                    // and we need 1 more step to operate "plus one" itself
                    // so we add 1 + s.len()
                    res += 1 + s.len() as i32;

                    return res;
                }

                // "plus one" operation
                res += 1;
            } else {
                s.pop();
                res += 1;
            }
        }

        res
    }
}