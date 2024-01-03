impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut prev: i32 = 0;
        let mut current: i32 = 0;

        let mut ans: i32 = 0;

        for s in bank.iter() {
            for byte in s.bytes() {
                match byte {
                    48 => continue,
                    _ => current += 1
                }
            }

            match prev {
                0 => {},
                _ => {
                    if current == 0 {
                        continue;
                    }
                    ans += prev * current;
                }
            }
            prev = current;
            current = 0;
        }

        ans
    }
}