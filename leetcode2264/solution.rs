impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let bytes = num.as_bytes();
        let mut left: usize = 0;
        let mut max: &str = "";

        for right in 1..bytes.len() {
            if bytes[right] != bytes[left] {
                left = right;
                continue;
            }
            if right - left == 2 {
                if &num[left..(right + 1)] > max {
                    max = &num[left..(right + 1)];
                }
            }
            
        }

        String::from(max)
    }
}