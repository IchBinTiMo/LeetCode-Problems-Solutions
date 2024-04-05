impl Solution {
    pub fn make_good(s: String) -> String {
        s.chars()
            .fold(String::new(), |mut result, c2| match (result.pop(), c2) {
                (None, c2) => c2.to_string(),
                (Some(c1), c2) if c1 != c2 && c1.to_lowercase().eq(c2.to_lowercase()) => result,
                (Some(c1), c2) => result + &c1.to_string() + &c2.to_string(),
            })
    }
}

// impl Solution {
//     pub fn make_good(mut s: String) -> String {
//         if s.len() <= 1 {
//             return s;
//         }

//         let mut stack: Vec<char> = Vec::new();

//         while let Some(c) = s.pop() {
//             if let Some(last) = stack.pop() {
//                 if c != last && c.to_lowercase().eq(last.to_lowercase()) {
//                     continue;
//                 } else {
//                     stack.push(last);
//                     stack.push(c);
//                 }
//             } else {
//                 stack.push(c);
//             }
//         }

//         stack.iter().rev().map(|&e| e as char).collect::<String>()
//     }
// }