impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels: Vec<u8> = vec![b'A', b'E', b'I', b'O', b'U', b'a', b'e', b'i', b'o', b'u'];
        let mut counts: Vec<i32> = vec![0; 10];
        let mut idx: Vec<usize> = vec![];
        let mut ans: Vec<u8> = Vec::with_capacity(s.len());

        for (i, b) in s.bytes().enumerate() {
            match b {
                b'A' => {counts[0] += 1; idx.push(i);},
                b'E' => {counts[1] += 1; idx.push(i);},
                b'I' => {counts[2] += 1; idx.push(i);},
                b'O' => {counts[3] += 1; idx.push(i);},
                b'U' => {counts[4] += 1; idx.push(i);},
                b'a' => {counts[5] += 1; idx.push(i);},
                b'e' => {counts[6] += 1; idx.push(i);},
                b'i' => {counts[7] += 1; idx.push(i);},
                b'o' => {counts[8] += 1; idx.push(i);},
                b'u' => {counts[9] += 1; idx.push(i);},
                _ => (),
            }
            ans.push(b);
        }

        let mut sv: Vec<(u8, i32)> = vowels.iter().zip(counts.iter()).filter(|(_, &c)| c > 0).map(|x| {(*x.0, *x.1)}).collect::<Vec<(u8, i32)>>();

        for i in idx.iter() {
            ans[*i] = sv[0].0;
            sv[0].1 -= 1;
            if sv[0].1 == 0 {
                sv.remove(0);
            }
        }


        String::from_utf8(ans).unwrap()
    }
}