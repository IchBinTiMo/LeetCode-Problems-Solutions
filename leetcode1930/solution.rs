impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut st, mut end) = ([-1; 26], [-1; 26]);
        for (i, c) in s.iter().enumerate() {
            if st[*c as usize - 'a' as usize] == -1 {
                st[*c as usize - 'a' as usize] = i as i32;
            }
            end[*c as usize - 'a' as usize] = i as i32;
        }
        let mut ans = 0;
        for i in 0..26 {
            if end[i] != -1 {
                let mut arr = [false; 26];
                for j in (st[i]+1)..end[i] {
                    if !arr[s[j as usize] as usize - 'a' as usize] {
                        arr[s[j as usize] as usize - 'a' as usize] = true;
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
// use std::collections::HashSet;
// use std::collections::HashMap;

// impl Solution {
//     pub fn count_palindromic_subsequence(s: String) -> i32 {
//         let mut map: HashMap<u8, HashSet<u8>> = HashMap::new();
//         let mut ans: i32 = 0;
//         let bytes: Vec<u8> = s.bytes().collect();
//         let mut count: Vec<(i32, i32)> = vec![(0, 0); 26];

//         for i in 0..1 {
//             count[bytes[i] as usize - 'a' as usize].0 += 1;
//         }
//         for i in 2..bytes.len() {
//             count[bytes[i] as usize - 'a' as usize].1 += 1;
//         }


//         for i in 1..bytes.len() - 1 {
//             let c = bytes[i];
//             for j in b'a'..=b'z' {
//                 let idx = (j - b'a') as usize;
//                 if count[idx].1 * count[idx].0 > 0 {
//                     match map.get_mut(&c) {
//                         Some(set) => {
//                             if !set.contains(&j) {
//                                 ans += 1;
//                                 set.insert(j);
//                             }
//                         },
//                         None => {
//                             ans += 1;
//                             map.insert(c, HashSet::new());
//                             map.get_mut(&c).unwrap().insert(j);
//                         }
                        
//                     }
//                 }
//             }
//             count[(c - b'a') as usize].0 += 1;
//             count[(bytes[i + 1]  - b'a') as usize].1 -= 1;
//         }
//         ans
//     }
// }