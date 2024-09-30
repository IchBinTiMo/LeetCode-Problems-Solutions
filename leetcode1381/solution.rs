/*
Solution:

Time: O(1) | Space: O(n)

Runtime: 4 ms | 87.50%
Memory: 3.11 MB | 62.50%

- n: maxSize
*/

struct CustomStack {
    max_size: usize,
    stack: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        CustomStack {
            max_size: maxSize as usize,
            stack: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        if let Some(val) = self.stack.pop() {
            val
        } else {
            -1
        }
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        let k: usize = k as usize;

        for i in 0..k.min(self.stack.len()) {
            self.stack[i] += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */