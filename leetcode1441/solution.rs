impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut idx = 1;
        let mut stack: Vec<i32> = vec![];

        let mut ans: Vec<String> = vec![];

        while stack != target {
            if !target.contains(&idx) {
                ans.push("Push".to_string());
                ans.push("Pop".to_string());
                idx += 1;
            }
            else {
                stack.push(idx);
                idx += 1;
                ans.push("Push".to_string());
            }
        }
        ans
    }
}