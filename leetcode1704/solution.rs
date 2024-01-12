impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowels: Vec<char> = vec!['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

        let l = s.len();

        let mut cnt: i32 = 0;

        for c in s[..(l / 2)].chars() {
            if vowels.contains(&c) {
                cnt += 1;
            }
        }

        for c in s[(l / 2)..].chars() {
            if vowels.contains(&c) {
                cnt -= 1;
            }
        }

        cnt == 0
    }
}