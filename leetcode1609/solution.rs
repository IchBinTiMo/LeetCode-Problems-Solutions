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
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        let mut level:i32 = 0;

        let mut row: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

        row.push(root);

        while !row.is_empty() {
            let mut next_row: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

            if level & 1 == 0 {
                let mut current: i32 = i32::MIN;

                for i in 0..row.len() {
                    if let Some(node) = &row[i] {
                        let n = node.borrow();

                        if n.val <= current {
                            return false;
                        }

                        if n.val & 1 == 0 {
                            return false;
                        }

                        current = n.val;

                        next_row.push(n.left.clone());
                        next_row.push(n.right.clone());
                    }
                }
            }

            if level & 1 == 1 {
                let mut current: i32 = i32::MAX;

                for i in 0..row.len() {
                    if let Some(node) = &row[i] {
                        let n = node.borrow();

                        if n.val >= current {
                            return false;
                        }

                        if n.val & 1 == 1 {
                            return false;
                        }
                        current = n.val;

                        next_row.push(n.left.clone());
                        next_row.push(n.right.clone());
                    }
                }

            }
            row = next_row;
            level += 1;

        }
        true
    }
}