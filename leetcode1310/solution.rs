/*
Solution:

Time: O(m + n) | Space: O(m + n)

Runtime: 10 ms | 100.00%
Memory: 4.09 MB | 42.86%

- m: length of 'arr'
- n: length of 'queries'
*/

impl Solution {
    pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut xors: Vec<i32> = vec![0];

        for i in 0..arr.len() {
            xors.push(xors[i] ^ arr[i]);
        }

        let mut res: Vec<i32> = Vec::new();

        for i in 0..queries.len() {
            let query = &queries[i];
            
            res.push(xors[query[0] as usize] ^ xors[query[1] as usize + 1]);
        }

        res
    }
}