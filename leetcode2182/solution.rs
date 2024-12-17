/*
Solution: 

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.67 MB | 63.67%

- n: length of 's'
*/

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut heap: Vec<(u8, i32)> = Vec::new();

        let mut freqs: [i32; 26] = [0; 26];

        for c in s.bytes() {
            freqs[(c - b'a') as usize] += 1;
        }

        for i in 0..26 {
            if freqs[i] > 0 {
                heap.push((b'a' + i as u8, freqs[i]));
            }
        }

        let mut res: String = String::new();

        let mut idx: usize = heap.len() - 1;

        while !heap.is_empty() {
            let (top, freq): (u8, i32) = heap[idx];

            let cnt: i32 = repeat_limit.min(freq);

            for _ in 0..cnt {
                res.push(top as char);
            }

            if cnt == freq {
                if freq == cnt {
                    heap.pop();
                    idx -= 1;
                } else {
                    heap[idx].1 -= cnt;
                }

                continue;
            }

            if idx - 1 > heap.len() {
                break;
            }

            let (next, fq): (u8, i32) = heap[idx - 1];

            res.push(next as char);

            if fq == 1 {
                heap.remove(idx - 1);
                idx -= 1;
            } else {
                heap[idx - 1].1 -= 1;
            }

            if freq == cnt {
                heap.pop();
                idx -= 1;
            } else {
                heap[idx].1 -= cnt;
            }
        }

        res
    }
}