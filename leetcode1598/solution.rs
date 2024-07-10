impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        /// Time O(N) | Space O(1)
        /// 
        /// where 'N' is the length of 'logs'
        let mut depth: i32 = 0;

        for log in logs.iter() {
            if log == "../" {
                if depth > 0 {
                    depth -= 1;
                }
            } else if log == "./" {
                continue;
            } else {
                depth += 1;
            }
        }

        depth
    }
}