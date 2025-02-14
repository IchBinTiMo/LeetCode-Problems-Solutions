/*
Solution: 

Time: O(n) | Space: O(n)

Runtime: 15 ms | 75.00%
Memory: 19.33 MB | -%

- n: # of queries
*/

struct ProductOfNumbers {
    nums: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        ProductOfNumbers {
            nums: Vec::new(),
        }
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums.clear();
        } else {
            if !self.nums.is_empty() {
                self.nums.push(num * self.nums[self.nums.len() - 1]);
            } else {
                self.nums.push(num);
            }
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let n: usize = self.nums.len();

        if n as i32 - k < 0 {
            return 0;
        } else if n as i32 - k == 0 {
            return self.nums[n - 1];
        } else {
            return self.nums[n - 1] / self.nums[n - 1 - k as usize];
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */