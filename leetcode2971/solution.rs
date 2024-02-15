impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();

        let mut sum: i64 = nums.iter().map(|&num| num as i64).sum();

        while let Some(num) = nums.pop() {
            let num = num as i64;

            sum -= num;

            if sum > num {
                return sum + num;
            }
        }

        -1
    }
}