impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return arr[0].max(arr[1]);
        }
        if k >= arr.len() as i32 {
            return *(arr.iter().max().unwrap());
        }
        let mut current = arr[0];
        let mut cnt = 0;

        for &n in &arr[1..] {
            if current > n {
                cnt += 1;
            }
            else {
                current = n;
                cnt = 1;
            }
            if cnt == k {
                return current;
            }
        }
        current
    }
}