/*
Solution: Sorting + Greedy

Time: O(n log n) | Space: O(q)

Runtime: 14 ms | 100.00%
Memory: 9.77 MB | 100.00%

- m: length of 'items'
- q: length of 'queries'
- n: max(m, q)
*/

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut indices: Vec<usize> = (0..queries.len()).collect::<Vec<usize>>();

        items.sort_unstable_by_key(|k| k[0]);
        indices.sort_unstable_by_key(|&k| queries[k]);

        let mut maxi: i32 = 0;

        let mut current: usize = 0;
        
        let mut res: Vec<i32> = vec![0; queries.len()];

        for i in 0..indices.len() {
            while current < items.len() && items[current][0] <= queries[indices[i]] {
                maxi = maxi.max(items[current][1]);
                current += 1;
            }

            res[indices[i]] = maxi;
        }

        res
    }
}