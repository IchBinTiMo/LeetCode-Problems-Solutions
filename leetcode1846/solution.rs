impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        if *arr.iter().max().unwrap() > arr.len() as i32{
            return arr.len() as i32;
        } else {
            let mut count: Vec<i32> = Vec::with_capacity(100000);
            count.resize(100000, 0);

            for value in arr.iter() {
                count[(value - 1) as usize] += 1;
            }

            let mut cnt: Vec<(usize, i32)> = count
                                                .iter()
                                                .enumerate()
                                                .filter(|(idx, val)| **val > 0)
                                                .map(|(idx, val)| (idx, *val))
                                                .collect::<Vec<(usize, i32)>>();

            let mut current: i32 = cnt[0].1;
            let mut ans: i32 = 0;

            for idx in 0..(cnt.len() - 1) {
                let num = cnt[idx].0;
                let c = cnt[idx].1;

                
                let next_num = cnt[idx + 1].0;
                let next_c = cnt[idx + 1].1;
                
                if (next_num - num) as i32 > arr.len() as i32 - current {
                    return arr.len() as i32 - current + (num + 1) as i32;
                } else {
                    current += next_c;
                    ans = next_num as i32;
                }
            }


            ans + 1
        }
    }
}