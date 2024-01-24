impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut tmp: i32 = 0;

        for &num in nums.iter() {
            tmp ^= num;
        }

        let mut shift: i32 = 0;

        loop {
            if tmp & 1 == 0 {
                shift += 1;
                tmp >>= 1;
            } else {
                break;
            }
        }

        let mut first: i64 = 0;
        let mut second: i64 = 0;

        for num in nums.iter().map(|&x| x as i64) {
            if num & (1 << shift) == 0 {
                first ^= num;
            } else {
                second ^= num;
            }
        }

        vec![first as i32, second as i32]
    }
}