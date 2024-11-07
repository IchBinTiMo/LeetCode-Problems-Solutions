/*
Solution: Bit Manipulation

Time: O(n) | Space: O(1)

Runtime: 28 ms | 100.00%
Memory: 3.47 MB | 54.55%

- n: length of 'candidates'
*/

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut bits: [i32; 32] = [0; 32];

        for &c in candidates.iter() {
            let mut c: i32 = c;
            let mut bit: usize = 0;

            while c > 0 {
                if c & 1 == 1 {
                    bits[bit] += 1;
                }
                bit += 1;
                c >>= 1;
            }
        }

        *bits.iter().max().unwrap()
    }
}