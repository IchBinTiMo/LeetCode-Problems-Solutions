impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut ans: i32 = 1;
        let mut current = capacity;

        for i in 0..plants.len() - 1 {
            ans += 1;
            current -= plants[i];

            if plants[i + 1] > current {
                ans += (i as i32 + 1) * 2;
                current = capacity;
            }
            
        }

        ans
    }
}