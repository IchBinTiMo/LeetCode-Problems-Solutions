impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        /// Backtracking
        /// 
        /// Time: O(2 ^ n) | Space: O(2 ^ n)
        let chars: Vec<char> = s.chars().collect::<Vec<char>>();

        let mut res: Vec<Vec<String>> = Vec::new();

        let mut path: Vec<String> = Vec::new();

        Self::backtracking(&mut res, &mut path, 0, &chars);

        res
    }

    fn backtracking(res: &mut Vec<Vec<String>>, path: &mut Vec<String>, current: usize, chars: &Vec<char>) {
        if current == chars.len() {
            res.push(path.to_vec());
        } else {
            for i in current..chars.len() {
                if Self::is_palindrome(&chars[current..=i]) {
                    path.push(chars[current..=i].to_vec().iter().collect::<String>());
                    Self::backtracking(res, path, i + 1, chars);
                    path.pop();
                }
            }
        }
    }

    fn is_palindrome(chars: &[char]) -> bool {
        if chars.len() & 1 == 1 {
            let mid: usize = chars.len() / 2;

            for i in 1..(chars.len() - mid) {
                if chars[mid + i] != chars[mid - i] {
                    return false;
                }
            }

            return true;

        } else {
            let mut right = chars.len() / 2;
            let mut left = chars.len() - right - 1;

            while right < chars.len() {
                if chars[right] == chars[left] {
                    right += 1;
                    left -= 1;
                } else {
                    return false;
                }
            }

            return true;
        }
    }
}