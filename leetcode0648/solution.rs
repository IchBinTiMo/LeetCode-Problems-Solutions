use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    is_word: bool,
    children: HashMap<char, TrieNode>
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default()
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current: &mut TrieNode = &mut self.root;

        for c in word.chars() {
            current = current.children.entry(c).or_default();
        }
        current.is_word = true;
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        /// Time: O(n * m) | Space: O(m)
        /// where n is the length of the sentence
        /// and m is the sum of the lengths of all words in the dictionary
        
        // create a trie
        let mut trie: Trie = Trie::new();

        // insert all words into the trie
        for word in dictionary.iter() {
            trie.insert(word);
        }

        // replace all words in the sentence
        sentence.split(" ").map(|word| {
            let mut current: &TrieNode = &trie.root;
            let mut path: String = String::new();

            for c in word.chars() {
                match current.children.get(&c) {
                    Some(next) => {
                        // if current character could be our next step
                        // then add it to the path
                        path.push(c);
                        if next.is_word {
                            // because we want the shortest prefix
                            // once path is a valid word in dictionary
                            // we can stop
                            return path;
                        }
                        current = next;
                    },
                    // break if current character couldn't be our next step
                    None => break
                }
            }

            word.to_string()
        }).collect::<Vec<String>>().join(" ")
    }
}

// use std::collections::HashMap;

// #[derive(Default, Debug)]
// struct TrieNode {
//     is_word: bool,
//     children: HashMap<char, TrieNode>
// }

// #[derive(Default, Debug)]
// pub struct Trie {
//     root: TrieNode
// }

// impl Trie {
//     pub fn new() -> Self {
//         Trie {
//             root: TrieNode::default()
//         }
//     }

//     pub fn insert(&mut self, word: &str) {
//         let mut current: &mut TrieNode = &mut self.root;

//         for c in word.chars() {
//             current = current.children.entry(c).or_default();
//         }
//         current.is_word = true;
//     }

//     pub fn contains(&self, word: &str) -> bool {
//         let mut current: &TrieNode = &self.root;

//         for c in word.chars() {
//             match current.children.get(&c) {
//                 Some(next) => current = next,
//                 None => return false
//             }
//         }

//         current.is_word
//     }
// }

// impl Solution {
//     pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
//         /// Time O(n * m) | Space O(m)
//         /// where n is the length of the sentence
//         /// and m is the sum of the lengths of all words in the dictionary
        
//         // create a trie
//         let mut trie: Trie = Trie::new();

//         // insert all words in the dictionary into the trie
//         for word in dictionary.iter() {
//             trie.insert(word);
//         }

//         // split the sentence into words
//         let mut sentence: Vec<String> = sentence.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

//         // brute force search
//         // replace each word in the sentence with the first word that is in the trie
//         for i in 0..sentence.len() {
//             let word: &str = &*sentence[i];
//             for j in 0..word.len() {
//                 if trie.contains(&word[0..=j]) {
//                     sentence[i] = (&word[0..=j]).to_string();
//                     break;
//                 }
//             }
//         }

//         sentence.join(" ")
//     }
// }