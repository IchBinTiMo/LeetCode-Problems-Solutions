impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let mut current = 1;
        let mut nums: Vec<i32> = Vec::new();

        for _ in 0..n {
            while nums.contains(&(k - current)) {
                current += 1;
            }

            nums.insert(0, current);
            current += 1;
        }

        nums.iter().sum()

    }
}