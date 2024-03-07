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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        Solution::dfs(&mut res, root, &mut Vec::new(), target_sum);
        res
    }

    fn dfs(ans: &mut Vec<Vec<i32>>, root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, target: i32) {

        if let Some(root_node) = root {
            let node = root_node.borrow();

            path.push(node.val);

            if node.left.is_none() && node.right.is_none() {
                if node.val == target {
                    ans.push(path.clone());
                }
            }

            Solution::dfs(ans, node.left.clone(), path, target - node.val);
            Solution::dfs(ans, node.right.clone(), path, target - node.val);

            path.pop();

        }

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
//     pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
//         if root.is_none() {
//             return Vec::new();
//         }
//         let mut res: Vec<Vec<i32>> = Vec::new();
//         Solution::dfs(&mut res, root, &mut Vec::new(), target_sum);
//         res
//     }

//     fn dfs(ans: &mut Vec<Vec<i32>>, root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, target: i32) {

//         if let Some(root_node) = root {
//             let node = root_node.borrow();

//             path.push(node.val);

//             if node.left.is_none() && node.right.is_none() {
//                 if node.val == target {
//                     ans.push(path.clone());
//                 }
//             }

//             Solution::dfs(ans, node.left.clone(), path, target - node.val);
//             Solution::dfs(ans, node.right.clone(), path, target - node.val);

//             path.pop();

//         }

//     }
// }

// // // Definition for a binary tree node.
// // // #[derive(Debug, PartialEq, Eq)]
// // // pub struct TreeNode {
// // //   pub val: i32,
// // //   pub left: Option<Rc<RefCell<TreeNode>>>,
// // //   pub right: Option<Rc<RefCell<TreeNode>>>,
// // // }
// // // 
// // // impl TreeNode {
// // //   #[inline]
// // //   pub fn new(val: i32) -> Self {
// // //     TreeNode {
// // //       val,
// // //       left: None,
// // //       right: None
// // //     }
// // //   }
// // // }
// // use std::rc::Rc;
// // use std::cell::RefCell;
// // impl Solution {
// //     pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
// //         if root.is_none() {
// //             return Vec::new();
// //         }
// //         let mut res: Vec<Vec<i32>> = Vec::new();
// //         Solution::dfs(&mut res, root, &mut Vec::new(), 0, target_sum);
// //         res
// //     }

// //     fn dfs(ans: &mut Vec<Vec<i32>>, root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, current: i32, target: i32) {

// //         if let Some(root_node) = root {
// //             let node = root_node.borrow();

// //             path.push(node.val);

// //             if node.left.is_none() && node.right.is_none() {
// //                 if current + node.val == target {
// //                     ans.push(path.clone());
// //                 }
// //             }

// //             Solution::dfs(ans, node.left.clone(), path, current + node.val, target);
// //             Solution::dfs(ans, node.right.clone(), path, current + node.val, target);

// //             path.pop();

// //         }

// //     }
// // }