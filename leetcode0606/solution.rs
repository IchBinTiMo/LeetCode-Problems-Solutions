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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans = String::new();

        Solution::preorder(root, &mut ans, 0);

        String::from(&ans[1..ans.len() - 1])
    }

    pub fn preorder(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut String, flag: u8) {
        match root {
            Some(node) => {
                ans.push_str("(");
                ans.push_str(&node.borrow().val.to_string());
                if node.borrow().right.clone().is_some() || node.borrow().left.clone().is_some() { 
                  Solution::preorder(node.borrow().left.clone(), ans, 0);

                }
                Solution::preorder(node.borrow().right.clone(), ans, 1);

                ans.push_str(")");
            },
            _ => {
                match flag {
                    0 => {
                        ans.push_str("()");
                        return;
                    },
                    _ => return
                }
            }
        }
    }
}