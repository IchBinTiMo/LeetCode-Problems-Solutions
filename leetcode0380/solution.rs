use std::collections::HashMap;
use rand::prelude::*;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet{
            map: HashMap::new(),
            list: Vec::new()
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.map.insert(val, self.list.len());
        self.list.push(val);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val) {
            Some(idx) => {
                self.list.swap_remove(idx);
                if idx < self.list.len() {
                    self.map.insert(self.list[idx], idx);
                }
                true
            },
            None => false
        }
    }
    
    fn get_random(&self) -> i32 {
        self.list[rand::thread_rng().gen_range(0, self.list.len())]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */