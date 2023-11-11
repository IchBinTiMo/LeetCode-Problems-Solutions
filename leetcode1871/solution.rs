use std::collections::VecDeque;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let min_jump: usize = min_jump as usize;
        let max_jump: usize = max_jump as usize;

        let v: Vec<char> = s.chars().collect();

        if v.last() == Some(&'1') {
            return false;
        }
        let mut right: usize = 0;
        let mut spot: VecDeque<usize> = VecDeque::new();

        spot.push_back(0);

        while let Some(left) = spot.pop_front() {
            let from = left + min_jump;
            let to = left + max_jump + 1 as usize;
            for i in from.max(right)..to.min(s.len()) {
                if v[i] == '0' {
                    if i == s.len() - 1 {
                        return true;
                    }
                    spot.push_back(i);

                }
            }
            right = to.max(right);
        }
        false
    }
}