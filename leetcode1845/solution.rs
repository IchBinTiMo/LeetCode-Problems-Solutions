use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct SeatManager {
  next: i32,
  ur: BinaryHeap<Reverse<i32>>
}


/** 
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl SeatManager {

  fn new(n: i32) -> Self {
    SeatManager {
        ur: BinaryHeap::new(),
        next: 1
    }
  }
  
  fn reserve(&mut self) -> i32 {
    if let Some(Reverse(x)) = self.ur.pop() {
        x
    } 
    else {
        let tmp = self.next;
        self.next += 1;
        tmp
    }
    
  }
  
  fn unreserve(&mut self, seat_number: i32) {
    self.ur.push(Reverse(seat_number));
  }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */