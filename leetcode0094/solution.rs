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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        if let Some(node) = root {
            let mut nd = node.borrow_mut();
            ans.extend(Solution::inorder_traversal(nd.left.take()));
            ans.push(nd.val);
            ans.extend(Solution::inorder_traversal(nd.right.take()));
        }
        ans
    }
}
// use std::rc::Rc;
// use std::cell::RefCell;
// impl Solution {
//     pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
//         let mut ans: Vec<i32> = vec![];
//         Solution::inorder(root, &mut ans);
//         ans
//     }

//     pub fn inorder(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
//         if let Some(node) = root {
//             let nd = node.borrow();
//             Solution::inorder(nd.left.clone(), ans);
//             ans.push(nd.val);
//             Solution::inorder(nd.right.clone(), ans);
//         }
//     }
// }