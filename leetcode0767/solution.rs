use std::collections::BinaryHeap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut cnt: [i32; 26] = [0; 26];

        for byte in s.bytes() {
            cnt[(byte - b'a') as usize] += 1;
        }

        let mut ans: String = String::new();

        let mut heap: BinaryHeap<(i32, char)> = BinaryHeap::new();

        for i in 0..26 {
            let i = i as usize;

            if cnt[i] == 0 {
                continue;
            } else {
                heap.push((cnt[i], char::from(i as u8 + b'a')));
            }
        }

        let mut prev: (i32, char) = (-1, 'a');


        while let Some((mut count, c)) = heap.pop() {
            if count as f64 > (s.len() as f64 / 2 as f64).ceil() {
                return String::new();
            }

            ans.push(c);
            count -= 1;

            if prev.0 > 0 {
                heap.push((prev.0, prev.1));
            }

            prev = (count, c);
        }
        ans
    }
}