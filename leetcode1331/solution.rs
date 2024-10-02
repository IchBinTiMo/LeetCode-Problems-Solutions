/*
Solution:

Time: O(n log n) | Space: O(n)

Runtime: 18 ms | 74.36%
Memory: 4.64 MB | 35.90%

- n: length of arr
*/

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty() {
            return Vec::new()
        }
        
        let n: usize = arr.len();
        let mut indices: Vec<(usize, i32)> = Vec::new();

        let mut current: i32 = 1;

        for i in 0..n {
            indices.push((i, 0));
        }

        indices.sort_unstable_by_key(|idx| arr[idx.0]);

        indices[0].1 = 1;

        for i in 1..n {
            current += (arr[indices[i].0] != arr[indices[i - 1].0]) as i32;
            indices[i].1 = current;
        }

        indices.sort_unstable_by_key(|idx| idx.0);

        indices.into_iter().map(|(old, new)| new).collect::<Vec<i32>>()
    }
}