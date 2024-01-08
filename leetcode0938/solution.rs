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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(rt) = root {
            let r = rt.borrow();
            if r.val >= low && r.val <= high {
                return r.val + 
                    Solution::range_sum_bst(r.left.clone(), low, high) + 
                    Solution::range_sum_bst(r.right.clone(), low, high); 

            } else if r.val < low {
                return Solution::range_sum_bst(r.right.clone(), low, high);
            } else if r.val > high {
                return Solution::range_sum_bst(r.left.clone(), low, high);
            }
        
        }
        0
    }
}