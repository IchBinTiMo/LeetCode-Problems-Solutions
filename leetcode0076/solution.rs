use std::collections::HashMap;

impl Solution 
{
    pub fn min_window(s: String, t: String) -> String 
    {
        let s_bytes = s.as_bytes();
        
        let mut counter: HashMap<u8,i32> = t
            .bytes()
            .fold(HashMap::new(), |mut cnt,ch| { 
                *cnt.entry(ch).or_default() += 1; 
                cnt 
            });
        
        let mut missing = counter.len() as i32; 
        let (mut left, mut right)  = (0, 0);
        let (mut wl, mut wr) = (0, s.len()+1);
        
        loop
        {
            if right < s_bytes.len() && missing > 0
            {
                if let Some(cnt) = counter.get_mut(&s_bytes[right])
                {
                    *cnt -= 1;
                    if *cnt == 0 { 
                        missing -= 1; 
                    }
                }
                right += 1;
            }
                
            else if left < s_bytes.len() && missing <= 0
            {
                if right - left < wr - wl { 
                    wl = left; 
                    wr = right; 
                }

                if let Some(cnt) = counter.get_mut(&s_bytes[left])
                {
                    if *cnt == 0 { 
                        missing += 1; 
                    }
                    *cnt += 1;
                }
                left += 1;
            }
            else { 
                break; 
            }
        }
        
        return if wr <= s_bytes.len() { 
            s[wl..wr].to_string() 
        } else { 
            String::new() 
        };
    }
}