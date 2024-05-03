impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        /// Two Pointers
        /// 
        /// Time: O(n) | Space: O(1)
        let version1: Vec<&str> = version1.split(".").collect();
        let version2: Vec<&str> = version2.split(".").collect();

        let mut left: usize = 0;
        let mut right: usize = 0;

        loop {
            let mut re_1_num: i32 = 0;
            let mut re_2_num: i32 = 0;

            match (version1.get(left), version2.get(right)) {
                (Some(re_1), Some(re_2)) => {
                    re_1_num = re_1.parse().unwrap();
                    re_2_num = re_2.parse().unwrap();
                },
                (Some(re_1), None) => {
                    re_1_num = re_1.parse().unwrap();
                },
                (_, Some(re_2)) => {
                    re_2_num = re_2.parse().unwrap();
                },
                (_, _) => {
                    return 0;
                },
            }

            if re_1_num > re_2_num {
                return 1;
            } else if re_1_num < re_2_num {
                return -1;
            }

            left += 1;
            right += 1;
        }

        1
    }
}