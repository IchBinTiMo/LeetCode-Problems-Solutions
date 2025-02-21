/*
Solution: DFS

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 5.89 MB | 50.00%

- n: # of nodes in 'root'
*/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

struct FindElements {
    values: HashSet<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut set: HashSet<i32> = HashSet::new();

        Self::dfs(&mut set, 0, root);

        FindElements{
            values: set
        } 
    }
    
    fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }

    fn dfs(set: &mut HashSet<i32>, val: i32, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            set.insert(val);

            Self::dfs(set, val * 2 + 1, node.left.take());
            Self::dfs(set, val * 2 + 2, node.right.take());
        }
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */