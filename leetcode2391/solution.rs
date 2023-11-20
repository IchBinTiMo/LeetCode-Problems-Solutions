impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut times: i32 = 0;
        let mut end: Vec<usize> = vec![usize::MIN; 3];

        for i in 0..garbage.len() {
            for c in garbage[i].chars() {
                times += 1;
                match c {
                    'G' => {
                        end[0] = end[0].max(i);
                    },
                    'M' => {
                        end[1] = end[1].max(i);
                    },
                    'P' => {
                        end[2] = end[2].max(i);
                    },
                    _ => continue,
                }
            }

        }

        for &d in end.iter() {
            times += travel[0..d].iter().sum::<i32>();
        }

        times

        
    }
}