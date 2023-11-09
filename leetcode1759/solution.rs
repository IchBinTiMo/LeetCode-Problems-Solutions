impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut ans = 0;

        let chars = s.chars().collect::<Vec<char>>();

        let mut memo: Vec<i32> = Vec::new();

        memo.push(1);

        while left <= right && right < chars.len() {
            right += 1;
            if right == chars.len() {
                ans += Solution::acc_memo(right - left, &mut memo);
                break
            }
            if chars[right] != chars[right - 1] {
                ans += Solution::acc_memo(right - left, &mut memo);
                left = right;
            }
        }

        ans
    }

    pub fn acc_memo(n: usize, memo: &mut Vec<i32>) -> i32 {
        if n > memo.len() {
            let res = Solution::acc_memo(n - 1, memo) + n as i32;
            memo.push(res);
        } 
        memo[n-1] % 1000000007
    }
}