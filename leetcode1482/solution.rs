impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        /// Binary Search
        /// 
        /// Time: O(nlogn) | Space: O(1)
        /// where n is the length of bloom_day
        if m as i64 * k as i64 > bloom_day.len() as i64 {
            return -1;
        }

        let mut low: usize = 1;
        let mut high: usize = *bloom_day.iter().max().unwrap() as usize;

        while low < high {
            let mid: usize = (low + high) / 2;

            if Self::bouquetable(&bloom_day, m, k, mid as i32) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low as i32
    }

    // function to check if we can get enough bouquets at day 'day'
    fn bouquetable(bloom_day: &Vec<i32>, m: i32, k: i32, day: i32) -> bool {
        let mut current_bouquet: i32 = 0;
        let mut current_flower: i32 = 0;

        for &b in bloom_day.iter() {
            if b <= day {
                current_flower += 1;

                if current_flower == k {
                    current_bouquet += 1;
                    current_flower = 0;
                }
            } else {
                current_flower = 0;
            }

            if current_bouquet >= m {
                return true;
            }
        }

        false
    }
}