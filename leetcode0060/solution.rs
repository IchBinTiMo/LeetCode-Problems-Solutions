impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut k = k;
        let mut fac: Vec<i32> = vec![1; (n + 1) as usize];

        let mut digit: Vec<i32> = Vec::new();

        for i in 1..=n {
            fac[i as usize] = i * fac[(i - 1) as usize];
        }

        for i in 1..=n {
            digit.push(i);
        }

        let mut ans = 0;

        for i in (0..=(n - 1)).rev() {
            if i == 0 {
                break;
            }
            let mut acc = 0;
            let mut cnt: usize = 0;

            while acc < k {
                acc += fac[i as usize];
                cnt += 1;
            }

            k -= acc;
            k += fac[i as usize];
            ans *= 10;
            ans += digit[cnt - 1];
            digit.remove(cnt - 1);
        }

        ans *= 10;
        ans += digit[0];

        ans.to_string()


    }


}