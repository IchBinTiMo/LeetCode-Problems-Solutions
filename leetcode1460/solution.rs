/*
Solution 1:

Time: O(n) | Space: O(1)

- n: length of arr
*/

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut freqs: [i32; 1001] = [0; 1001];

        for i in 0..arr.len() {
            freqs[target[i] as usize] += 1;
            freqs[arr[i] as usize] -= 1;
        }

        for i in 0..1001 {
            if freqs[i] != 0 {
                return false;
            }
        }

        true
    }
}