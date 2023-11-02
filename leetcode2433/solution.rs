impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = pref.clone();

        for i in 1..ans.len() {
            ans[i] = pref[i - 1] ^ pref[i];
        }

        ans
    }
}