impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        // monotonic stack
        let mut stack: Vec<char> = Vec::new();

        for c in num.chars() {
            // if the last element is bigger than current one, we need to pop it
            while let Some(&last) = stack.last() {
                if k > 0 && last > c {
                    stack.pop();
                    k -= 1;
                } else {
                    break;
                }
            }
            stack.push(c);
        }

        // pop the rest
        while k > 0 {
            stack.pop();
            k -= 1;
        }

        let mut start: usize = 0;

        // remove leading zeros
        while start < stack.len() && stack[start] == '0' {
            start += 1;
        }

        if start == stack.len() {
            return String::from("0");
        }

        stack[start..].iter().collect::<String>()
    }
}