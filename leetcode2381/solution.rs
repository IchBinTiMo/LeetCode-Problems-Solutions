/*
Solution: Prefix Sum

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 6.05 MB | 9.09%

- n: length of 's'
*/

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n: usize = s.len();

        let mut accs: Vec<i32> = vec![0; n + 1];

        for sh in shifts.iter() {
            let left: usize = sh[0] as usize;
            let right: usize = (sh[1] + 1) as usize;
            let dir: i32 = if sh[2] == 1 {1} else {-1};

            accs[left] += dir;
            accs[right] -= dir;
        }

        let s = s.as_bytes();

        let mut res: String = String::new();

        let mut acc: i32 = 0;

        for i in 0..n {
            acc += accs[i];

            acc = ((acc % 26) + 26) % 26;

            let c: u8 = (s[i] - b'a' + acc as u8) % 26 + b'a';

            res.push(c as char);
        }

        res
    }
}

/*
Solution: Brute Force

Time: O(m + n) | Space: O(m)

Runtime: 25 ms | 9.09%
Memory: 12.35 MB | 0.00%

- m: length of 's'
- n: length of 'shifts'
*/

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let m: usize = s.len();
        let n: usize = shifts.len();

        let mut lefts: Vec<Vec<usize>> = vec![Vec::new(); m];
        let mut rights: Vec<Vec<usize>> = vec![Vec::new(); m];

        for i in 0..n {
            let left: usize = shifts[i][0] as usize;
            let right: usize = shifts[i][1] as usize;

            lefts[left].push(i);
            rights[right].push(i);
        }

        let s = s.as_bytes();

        let mut res: String = String::new();

        let mut acc: i32 = 0;

        for i in 0..m {
            for &idx in lefts[i].iter() {
                let dir: i32 = if shifts[idx][2] == 1 {1} else {-1};

                acc += dir;
            }

            let step: i32 = ((acc % 26) + 26) % 26;

            let c: u8 = (s[i] - b'a' + step as u8) % 26 + b'a';

            res.push(c as char);

            for &idx in rights[i].iter() {
                let dir: i32 = if shifts[idx][2] == 1 {1} else {-1};

                acc -= dir;
            }
        }

        res
    }
}