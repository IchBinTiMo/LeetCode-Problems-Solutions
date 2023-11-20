impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut count1: Vec<i32> = vec![0; 26];
        let mut count2: Vec<i32> = vec![0; 26];

        let mut flag = 0;

        for b in s1.bytes() {
            count1[(b - b'a') as usize] += 1;
        }

        for b in s2.bytes() {
            count2[(b - b'a') as usize] += 1;
        }

        let mut count1: Vec<(usize, i32)> = count1.iter().enumerate().filter(|(idx, x)| *x > &0).map(|(idx, x)| (idx, *x)).collect();
        let mut count2: Vec<(usize, i32)> = count2.iter().enumerate().filter(|(idx, x)| *x > &0).map(|(idx, x)| (idx, *x)).collect();

        
        Solution::helper(count1.clone(), count2.clone()) || Solution::helper(count2, count1)
    }

    pub fn helper(count1: Vec<(usize, i32)>, count2: Vec<(usize, i32)>) -> bool {
        let mut count1 = count1;
        let mut count2 = count2;
        
        while count1.len() > 0 && count2.len() > 0 {
            if count1[0].0 < count2[0].0 {
                return false;
            }

            let cnt = count1[0].1.min(count2[0].1);

            count1[0].1 -= cnt;
            count2[0].1 -= cnt;

            if count1[0].1 == 0 {
                count1.remove(0);
            }

            
            if count2[0].1 == 0 {
                count2.remove(0);
            }

        }

        true
    }
}
