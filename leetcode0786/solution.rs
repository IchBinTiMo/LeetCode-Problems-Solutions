impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        /// Brute Force
        /// 
        /// Time: O(n^2) | Space: O(n)
        let n: usize = arr.len();

        let mut fractions: Vec<(f64, Vec<i32>)> = Vec::new();

        for i in 0..n {
            for j in (i + 1)..n {
                fractions.push(((arr[i] as f64) / (arr[j] as f64), vec![arr[i], arr[j]]));
            }
        }

        fractions.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        fractions[(k - 1) as usize].clone().1
    }
}