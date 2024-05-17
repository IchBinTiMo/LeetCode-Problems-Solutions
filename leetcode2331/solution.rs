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
    pub fn evaluate_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        /// DFS
        /// 
        /// Time O(N) | Space O(N)
        let mut ops: Vec<i32> = Vec::new();

        Self::traversal(root, &mut ops);
        
        ops.pop().unwrap() != 0
    }

    fn traversal(root: Option<Rc<RefCell<TreeNode>>>, ops: &mut Vec<i32>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            Self::traversal(node.left.take(), ops);
            Self::traversal(node.right.take(), ops);
            
            if node.val == 2 || node.val == 3 {
                let right: i32 = ops.pop().unwrap();
                let left: i32 = ops.pop().unwrap();

                ops.push(if node.val == 2 {left | right} else {left & right});
            } else {
                ops.push(node.val);
            }
        }
    }
}