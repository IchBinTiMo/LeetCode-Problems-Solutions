impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut trusting: Vec<usize> = vec![0; n];
        let mut trusted: Vec<usize> = vec![0; n];

        for t in trust {
            trusting[t[0] as usize - 1] += 1;
            trusted[t[1] as usize - 1] += 1;
        }

        for i in 0..n {
            if trusting[i] == 0 && trusted[i] == n - 1 {
                return i as i32 + 1;
            }
        }

        -1
    }
}