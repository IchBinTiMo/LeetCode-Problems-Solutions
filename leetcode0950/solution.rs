use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        /// sort the deck and do each step reversely
        let mut res: VecDeque<i32> = VecDeque::new();

        deck.sort_unstable();

        while let Some(card) = deck.pop() {
            // move the bottom card to the top
            if let Some(bottom) = res.pop_back() {
                res.push_front(bottom);
            }

            res.push_front(card);
        }

        /// convert VecDeque to Vec, equals to res.into()
        Vec::from_iter(res.into_iter())
    }
}