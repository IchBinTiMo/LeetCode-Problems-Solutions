impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut counter: [i32; 75] = [0; 75];

        for byte in s.bytes() {
            counter[(byte - b'0') as usize] += 1;
        }

        let mut sorted_indices: Vec<usize> = (0..75)
            .filter(|&i| counter[i] > 0)
            .collect();

        sorted_indices.sort_unstable_by_key(|&i| -counter[i]);

        let mut ans: String = String::new();

        for &idx in sorted_indices.iter() {
            for _ in 0..counter[idx] {
                ans.push((b'0' + idx as u8) as char);
            }
        }

        ans
    }
}