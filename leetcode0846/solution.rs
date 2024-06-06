use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        /// Time: O(n log n) | Space: O(n)
        /// where n is the length of hand
        
        // if the number of cards is not divisible by group size, it's impossible to form a straight hand
        if (hand.len() as i32) % group_size != 0 {
            return false;
        }

        let mut ht: HashMap<i32, i32> = HashMap::new();

        // count the number of each card
        // time: O(n) | space: O(n)
        for &num in hand.iter() {
            ht.entry(num).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        // reassign hand without duplicated values
        let mut hand: Vec<i32> = ht.keys().map(|&n| n).collect();

        // sort the hand
        // time: O(n log n) | space: O(1)
        hand.sort_unstable();

        let mut i: usize = 0;

        // check if there are enough cards to form a straight hand
        while i < hand.len() {
            if *ht.get(&hand[i]).unwrap() == 0 {
                i += 1;
                continue;
            }

            // check if there are enough cards to form a straight hand
            for j in 0..group_size {
                if let Some(&num) = ht.get(&(hand[i] + j)) {
                    if num == 0 {
                        // lack of cards with value hand[i] + j
                        return false;
                    }
                } else {
                    // lack of cards with value hand[i] + j
                    return false;
                }
            }

            // remove used cards
            for j in 0..group_size {
                ht.entry(hand[i] + j).and_modify(|cnt| *cnt -= 1);
            }
        }
        true
    }
}