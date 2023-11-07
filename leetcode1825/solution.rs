use std::collections::BTreeMap;

struct MKAverage {
    m: usize,
    k: i32,
    sum: i32,
    stream: Vec<i32>,
    sorted: BTreeMap<i32, i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {

    fn new(m: i32, k: i32) -> Self {
        MKAverage{
            m: m as usize,
            k: k,
            sum: 0,
            stream: Vec::new(),
            sorted: BTreeMap::new()
        }
    }
    
    fn add_element(&mut self, num: i32) {
        self.stream.push(num);
        *self.sorted.entry(num).or_insert(0) += 1;
        self.sum += num;
        if self.stream.len() > self.m {
            let rem = self.stream.remove(0);
            *self.sorted.get_mut(&rem).unwrap() -= 1;
            if *self.sorted.get(&rem).unwrap() == 0 {
                self.sorted.remove(&rem);
            }
            self.sum -= rem;
        }
    }
    
    fn calculate_mk_average(&self) -> i32 {
        if self.stream.len() < self.m {
          return -1;
        }

        let mut k_sum = 0;
        let mut k_times = self.k;
        for(num, count) in self.sorted.iter().take(self.k as usize) {
            if k_times == 0 {
            break;
            }
            if *count <= k_times {
            k_sum += *num * *count;
            k_times -= *count;
            }
            else {
            k_sum += *num * k_times;
            k_times = 0;
            }
        }

        k_times = self.k;

        for(num, count) in self.sorted.iter().rev().take(self.k as usize) {
            if k_times == 0 {
            break;
            }
            if *count <= k_times {
            k_sum += *num * *count;
            k_times -= *count;
            }
            else {
            k_sum += *num * k_times;
            k_times = 0;
            }
        }

        let ans = (self.sum - k_sum as i32) / (self.m as i32 - 2 * self.k) as i32;
        ans
        
        
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */