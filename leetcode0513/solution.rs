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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans: i32 = 0;

        Solution::traversal(&mut ans, root, 1, &mut 0);

        ans
    }

    fn traversal(ans: &mut i32, root: Option<Rc<RefCell<TreeNode>>>, current_depth: i32, max_depth: &mut i32) {
        if let Some(node) = root {
            let n = node.borrow();
            if current_depth > *max_depth {
                *ans = n.val;
                *max_depth = current_depth;
            }

            Solution::traversal(ans, n.left.clone(), current_depth + 1, max_depth);
            Solution::traversal(ans, n.right.clone(), current_depth + 1, max_depth);

        }
    }
}