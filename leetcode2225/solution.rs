impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cnt: [Option<i32>; 100001] = [None; 100001];

        for mch in matches.iter() {
            let winner = mch[0] as usize;
            let loser = mch[1] as usize;

            if cnt[winner].is_none() {
                cnt[winner] = Some(0);
            }

            if let Some(c) = cnt[loser] {
                cnt[loser] = Some(c + 1);
            } else {
                cnt[loser] = Some(1);
            }
        }

        let mut ans: Vec<Vec<i32>> = vec![Vec::new(); 2];

        for i in 0..cnt.len() {
            if let Some(c) = cnt[i] {
                match c {
                    0 => ans[0].push(i as i32),
                    1 => ans[1].push(i as i32),
                    _ => continue
                }
            }
        }

        ans
    }
}