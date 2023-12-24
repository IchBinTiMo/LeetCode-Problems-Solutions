impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for (i, byte) in s.bytes().enumerate() {
            match i % 2 {
                0 => {
                    match byte {
                        48 => b += 1,
                        _ => a += 1
                    }
                },
                _ => {
                    match byte {
                        48 => a += 1,
                        _ => b += 1
                    }
                }
            }
        }

        a.min(b)
        
    }
}