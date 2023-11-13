impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity((1 << n) as usize);
        for i in 0..(1 << n) {
            ans.push(i ^ (i >> 1));
        }
        ans

    }
}
