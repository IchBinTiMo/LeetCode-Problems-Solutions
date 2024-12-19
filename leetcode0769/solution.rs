/*
Solution: Sliding Window

Time: O(n log n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.26 MB | 12.50%

- n: length of 'arr'
*/

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut v: Vec<i32> = Vec::new();
        let mut w: Vec<i32> = Vec::new();


        let mut res: i32 = 0;

        for i in 0..arr.len() {
            let num: i32 = arr[i];

            v.push(i as i32);
            w.push(arr[i]);

            w.sort_unstable();

            if v == w {
                res += 1;
                v.clear();
                w.clear();
            }
        }

        res
    }
}