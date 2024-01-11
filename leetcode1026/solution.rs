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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::traverse(root, i32::MAX, i32::MIN, 0)
    }

    pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>, mut low: i32, mut high: i32, mut mx: i32) -> i32 {
        if let Some(rt) = root {
            let r = rt.borrow();

            low = low.min(r.val);
            high = high.max(r.val);

            mx = mx.max(high - low);

            mx = Solution::traverse(r.left.clone(), low, high, mx);
            mx = Solution::traverse(r.right.clone(), low, high, mx);

        }

        mx
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
//     pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut v1 = root.as_ref().unwrap().borrow().val.clone();
//         let mut v2 = root.as_ref().unwrap().borrow().val.clone();

//         Solution::traverse(root.clone(), v1, v2, 0)
//     }

//     pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>, mut low: i32, mut high: i32, mut mx: i32) -> i32 {
//         if let Some(rt) = root {
//             let r = rt.borrow();

//             if r.val < low {
//                 low = r.val;
//             } else if r.val > high {
//                 high = r.val;
//             }

//             mx = mx.max(high - low);

//             mx = Solution::traverse(r.left.clone(), low, high, mx);
//             mx = Solution::traverse(r.right.clone(), low, high, mx);

//         }
        
//         mx
//     }
// }