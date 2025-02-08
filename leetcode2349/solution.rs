/*
Solution:

Time: O(n log n) | Space: O(n)

Runtime: 38 ms | 100.00%
Memory: 56.96 MB | -%
*/

use std::collections::{HashMap, BinaryHeap};

struct NumberContainers {
    heaps: HashMap<i32, BinaryHeap<i32>>,
    indices: HashMap<i32, i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {

    fn new() -> Self {
        NumberContainers {
            heaps: HashMap::new(),
            indices: HashMap::new()
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        let mut prev: i32 = -1;

        if let Some(&tmp) = self.indices.get(&index) {
            prev = tmp;
        }

        self.indices.insert(index, number);
        self.heaps.entry(number).or_insert(BinaryHeap::new()).push(-index);

        if let Some(heap) = self.heaps.get_mut(&prev) {
            while let Some(&idx) = heap.peek() {
                if let Some(&num) = self.indices.get(&(-idx)) {
                    if prev == num {
                        break;
                    } else {
                        heap.pop();
                    }
                }
            }

            if heap.is_empty() {
                self.heaps.remove(&prev);
            }
        }
    }
    
    fn find(&self, number: i32) -> i32 {
        if let Some(heap) = self.heaps.get(&number) {
            -heap.peek().unwrap()
        } else {
            -1
        }
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */