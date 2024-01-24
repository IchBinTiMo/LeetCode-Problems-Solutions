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
    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans: i32 = 0;

        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, i32)> = vec![(root, 0)];

        while let Some((r, mut path)) = stack.pop() {
            if let Some(node) = r {
                let n = node.borrow();

                path ^= 1 << n.val;

                if n.left.is_none() && n.right.is_none() {
                    if path & (path - 1) == 0 {
                        ans += 1;
                    }
                } else {
                    stack.push((n.left.clone(), path));
                    stack.push((n.right.clone(), path));
                }
            }
        }

        ans
    }
}

// // Definition for a binary tree node.
// // #[derive(Debug, PartialEq, Eq)]
// // pub struct TreeNode {
// //   pub val: i32,
// //   pub left: Option<Rc<RefCell<TreeNode>>>,
// //   pub right: Option<Rc<RefCell<TreeNode>>>,
// // }
// // 
// // impl TreeNode {
// //   #[inline]
// //   pub fn new(val: i32) -> Self {
// //     TreeNode {
// //       val,
// //       left: None,
// //       right: None
// //     }
// //   }
// // }
// use std::rc::Rc;
// use std::cell::RefCell;
// impl Solution {
//     pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut ans: i32 = 0;

//         let mut occ: i32 = 0;

//         Solution::dfs(&mut ans, root, &mut occ);

//         ans
//     }

//     pub fn dfs(ans: &mut i32, root: Option<Rc<RefCell<TreeNode>>>, occ: &mut i32) {
//         if let Some(node) = root {
//             let n = node.borrow();

//             *occ = *occ ^ (1 << n.val);

//             if n.left.is_none() && n.right.is_none() {

//                 if *occ & (*occ - 1) == 0 {
//                     *ans += 1;
//                 }

//                 *occ = *occ ^ (1 << n.val);

//                 return;
//             }

//             Solution::dfs(ans, n.left.clone(), occ);
//             Solution::dfs(ans, n.right.clone(), occ);
         
//             *occ = *occ ^ (1 << n.val);
//         }
//     }
// }