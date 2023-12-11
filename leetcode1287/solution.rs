impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let l = arr.len();
        let mut cnt: usize = 0;
        let mut ans: i32 = arr[0];

        for i in 1..l {
            if arr[i] == arr[i - 1] {
                cnt += 1;
            } else {
                cnt = 1;
            }

            if cnt > l / 4 {
                return ans;
            }

            ans = arr[i];
        }

        arr[0]
    }
}