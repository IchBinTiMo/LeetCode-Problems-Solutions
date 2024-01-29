struct MyQueue {
    read: Vec<i32>,
    write: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue {
            read: Vec::new(),
            write: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.write.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.read = self.write.clone();
        self.read.reverse();
        let ret: i32 = self.read.pop().unwrap();
        self.write = self.read.clone();
        self.write.reverse();
        ret
    }
    
    fn peek(&mut self) -> i32 {
        *self.write.first().unwrap()
    }
    
    fn empty(&mut self) -> bool {
        self.write.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */