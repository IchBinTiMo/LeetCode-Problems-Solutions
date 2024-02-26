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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {

        if p.is_none() && q.is_none() {
            return true;
        }

        if let Some(p_node) = p {
            if let Some(q_node) = q {
                let pn = p_node.borrow();
                let qn = q_node.borrow();

                if pn.val == qn.val {
                    return Solution::is_same_tree(pn.left.clone(), qn.left.clone()) && Solution::is_same_tree(pn.right.clone(), qn.right.clone());
                }

                return false;
            }

            return false
        }

        return false;
    }
}