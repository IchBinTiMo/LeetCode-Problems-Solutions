impl Solution {
    pub fn length_of_last_word(mut s: String) -> i32 {
        let mut res: i32 = 0;
        let mut flag: bool = false; // to check if the last character is not ' '

        while let Some(c) = s.pop() {
            if c == ' ' && flag == false {
                continue;
            } else if c == ' ' && flag {
                break;
            } else {
                res += 1;
                // set flag to true
                flag = true;
            }
        }

        res
    }
}