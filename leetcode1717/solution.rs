impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        /// Time: O(n) | Space: O(n)
        /// 
        /// where 'n' is the length of 's'
        let mut chars: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut res: i32 = 0;

        if x > y {
            chars = Self::helper(&chars, true, x, &mut res);
            chars = Self::helper(&chars, false, y, &mut res);
        } else {
            chars = Self::helper(&chars, false, y, &mut res);
            chars = Self::helper(&chars, true, x, &mut res);
        }

        res
    }

    fn helper(chars: &Vec<char>, x: bool, step: i32, sum: &mut i32) -> Vec<char> {
        let mut ret: Vec<char> = Vec::new();

        if x {
            for &c in chars.iter() {
                if !ret.is_empty() && *ret.last().unwrap() == 'a' && c == 'b' {
                    ret.pop();
                    *sum += step;
                } else {
                    ret.push(c);
                }
            }
        } else {
            for &c in chars.iter() {
                if !ret.is_empty() && *ret.last().unwrap() == 'b' && c == 'a' {
                    ret.pop();
                    *sum += step;
                } else {
                    ret.push(c);
                }
            }
        }

        ret
    }
}