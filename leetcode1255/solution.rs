impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, scores: Vec<i32>) -> i32 {
        /// Backtracking
        /// 
        /// Time: O(2^N) | Space: O(N)
        /// where N is the length of words
        let mut res: i32 = 0;

        // transform letters to Vec<i32> with length of 26
        // and each element is the count of that letter from a to z, respectively
        // 97 is the ascii value of 'a'
        let mut letters: Vec<i32> = letters.iter().fold(vec![0; 26], |mut acc, &c| {acc[(c as usize) - 97] += 1; acc});

        // start backtracking
        Self::backtracking(&mut res, 0, 0, &words, &mut letters, &scores);

        res
    }


    // backtracking
    fn backtracking(res: &mut i32, current_score: i32, index: usize, words: &Vec<String>, letters: &mut Vec<i32>, scores: &Vec<i32>) {
        if *res < current_score {
            *res = current_score;
        }

        for i in index..words.len() {
            let score = Self::get_score(&words[i], letters, scores);

            Self::backtracking(res, current_score + score, i + 1, words, letters, scores);

            // score != 0 means we consume the word
            // so we need to return the letters used by the word
            // when the this iteration is over
            if score != 0 {
                Self::return_letters(&words[i], letters);
            }
        }
    }

    // count the score of the given word
    // return 0 if the word can not be formed by remained given letters
    fn get_score(word: &String, letters: &mut Vec<i32>, scores: &Vec<i32>) -> i32 {
        let mut remained: Vec<i32> = letters.clone();

        let mut score: i32 = 0;

        for c in word.chars() {
            if remained[(c as usize) - 97] == 0 {
                return 0;
            } else {
                remained[(c as usize) - 97] -= 1;
                score += scores[(c as usize) - 97];
            }
        }

        *letters = remained;

        score
    }

    // return the letters used by the word
    fn return_letters(word: &String, letters: &mut Vec<i32>) {
        for c in word.chars() {
            letters[(c as usize) - 97] += 1;
        }
    }
}