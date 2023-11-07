impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        fn bt(cookies: &Vec<i32>, current: &mut Vec<i32>, ans: &mut i32, start: usize, bags: usize, kids: usize) {
            if start == bags {
                let res = *current.iter().max().unwrap();
                (*ans) = (*ans).min(res);
                return;
            }

            for k in 0..kids {
                current[k] += cookies[start];
                if current[k] < *ans{
                    bt(cookies, current, ans, start + 1, bags, kids);

                }
                current[k] -= cookies[start];
            }
        }

        let mut ans = i32::MAX;
        let mut current: Vec<i32> = vec![0;k as usize];
        current[0] = cookies[0];

        bt(&cookies, &mut current, &mut ans, 1, cookies.len(), k as usize);

        ans


    }
}