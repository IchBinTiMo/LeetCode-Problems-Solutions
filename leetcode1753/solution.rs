impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut v = vec![a, b, c];
        v.sort();
        if v[0] + v[1] > v[2]{
            (a + b + c) / 2
        } else {
            v[0] + v[1]
        }
    }
}