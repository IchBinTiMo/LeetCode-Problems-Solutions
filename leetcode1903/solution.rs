impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut right: usize = num.len() - 1;

        while right <= 100000 {
          if (&num[right..=right]).parse::<i32>().unwrap() % 2 == 1 {
              return String::from(&num[0..=right]);
          }
          right -= 1;
        }

        String::new()

    }
}