impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        /// Binary Search
        /// 
        /// Time: O(n log n + n log k) | Space: O(1)
        /// where n is the length of position
        /// and k is max(postioin) - min(position)
        position.sort_unstable();

        let mut low: i32 = 0; // the minimum distance between two balls
        let mut high: i32 = position[position.len() - 1]; // the maximum distance between two balls

        let mut res: i32 = i32::MIN;

        while low < high {
            let mid: i32 = (low + high) / 2;

            if Self::placable(&position, mid, m) {
                low = mid + 1;
                res = mid;
            } else {
                high = mid;
            }
        }
        res
    }

    fn placable(position: &Vec<i32>, dist: i32, balls: i32) -> bool {
        /// put balls on the position
        /// so that the distance between each pair of adjacent balls is at least 'dist'
        /// if we can put at least 'balls' balls
        /// then return true
        /// otherwise return false
        let mut count: i32 = 1;

        let mut prev: i32 = position[0];

        for i in 1..position.len() {
            if position[i] - prev >= dist {
                count += 1;
                prev = position[i];
            }
            
            if count >= balls {
                return true;
            }
        }

        false
    }
}