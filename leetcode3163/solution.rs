/*
Solution: String

Time: O(n) | Space: O(n)

Runtime: 6 ms | 100.00%
Memory: 3.47 MB | 50.00%

- n: length of 'word'
*/

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut prev: char = '0';
        let mut cnt: u8 = 48;

        let mut res: Vec<char> = Vec::new();

        for c in word.chars() {
            if prev != '0' && c != prev {
                res.push(cnt as char);
                res.push(prev);
                cnt = 49;
                prev = c;
            } else {
                if cnt == 57 {
                    res.push(cnt as char);
                    res.push(c);
                    cnt = 48;
                }
                cnt += 1;
                prev = c;
            }
        }

        res.push(cnt as char);
        res.push(prev);

        res.iter().collect::<String>()
    }
}