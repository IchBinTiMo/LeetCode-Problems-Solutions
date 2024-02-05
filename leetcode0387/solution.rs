impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut occ: Vec<Vec<i32>> = vec![Vec::new(); 26];

        for (idx, byte) in s.bytes().enumerate() {
            occ[(byte - b'a') as usize].push(idx as i32);
        }

        let mut ans: i32 = i32::MAX;

        for i in 0..26 {
            if occ[i].len() != 1 {
                continue;
            }

            ans = occ[i][0].min(ans);
        }

        return if ans == i32::MAX {
            -1
        } else{
            ans
        }
    }
}