/*
Solution: Sliding Window

Time: O(n) | Space: O(1)

Runtime: 7 ms | 100.00%
Memory: 2.58 MB | 25.00%

- n: length of 's'
*/

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect::<Vec<char>>();

        let mut at_most: [i32; 3] = [0; 3];

        for &ch in s.iter() {
            match ch {
                'a' => at_most[0] += 1,
                'b' => at_most[1] += 1,
                'c' => at_most[2] += 1,
                _ => ()
            }
        }

        // if occurrences of any of 'a', 'b', and 'c' are not enough, return -1
        for i in 0..3 {
            if at_most[i] < k {
                return -1;
            }

            at_most[i] -= k;
        }

        let mut res: usize = 0;

        let mut left: usize = 0;

        let mut current: [i32; 3] = [0; 3]; // [a, b, c]

        for (right, &ch) in s.iter().enumerate() {
            let mut r: usize = 3;
            let mut valid: bool = true;

            match ch {
                'a' => r = 0,
                'b' => r = 1,
                'c' => r = 2,
                _ => ()
            }

            current[r] += 1;

            // if occurrences of current character is more than at_most, set 'valid' to false
            if current[r] > at_most[r] {
                valid = false;
            }

            // if valid, update 'res'
            if valid {
                res = res.max(right - left + 1);
            }

            // if not valid, slide the window
            while !valid {
                let mut l: usize = 3;

                match s[left] {
                    'a' => l = 0,
                    'b' => l = 1,
                    'c' => l = 2,
                    _ => ()
                }

                current[l] -= 1;
                left += 1;

                if l == r {
                    valid = true;
                }
            }
        }

        (s.len() - res) as i32

    }
}