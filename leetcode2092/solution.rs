impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        meetings.sort_unstable_by_key(|m| m[2]);

        let n = n as usize;

        let mut has_secret: Vec<bool> = vec![false; n as usize];

        has_secret[0] = true;
        has_secret[first_person as usize] = true;

        let mut left: usize = 0;

        while left < meetings.len() {
            let time: i32 = meetings[left][2];
            let mut sharing: bool = true;
            let mut right: usize = left;

            while sharing {
                sharing = false;
                right = left;

                while right < meetings.len() && time == meetings[right][2] {
                    let x = meetings[right][0] as usize;
                    let y = meetings[right][1] as usize;

                    if has_secret[x] ^ has_secret[y] {
                        sharing = true;
                        has_secret[x] = true;
                        has_secret[y] = true;
                    }
                    right += 1;
                }
            }
            left = right;
            
        }

        (0..n).filter(|&i| has_secret[i]).map(|idx| idx as i32).collect()
    }
}