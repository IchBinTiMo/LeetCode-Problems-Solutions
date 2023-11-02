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
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            if let Some(node) = root {
                let l = dfs(node.borrow().left.clone());
                let r = dfs(node.borrow().right.clone());
                let sum = node.borrow().val + l.1 + r.1;
                if node.borrow().val == sum / (l.2 + r.2 + 1) {
                    (l.0 + r.0 + 1, sum, l.2 + r.2 + 1)
                }
                else{
                    (l.0 + r.0, sum, l.2 + r.2 + 1)
                    
                }
            }
            else{
                (0, 0, 0)
            }
        }
        dfs(root).0
    }
}