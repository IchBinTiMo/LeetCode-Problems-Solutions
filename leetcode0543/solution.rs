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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(root).0
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }

        let root_node = root.unwrap();
        let node = root_node.borrow();

        let left: (i32, i32) = Solution::dfs(node.left.clone());
        let right: (i32, i32) = Solution::dfs(node.right.clone());

        let diameter: i32 = left.0.max(right.0.max(left.1 + right.1));
        let height: i32 = left.1.max(right.1) + 1;

        (diameter, height)
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
//     pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut ans: i32 = i32::MIN;

//         Solution::dfs(&mut ans, root);

//         ans
//     }

//     fn dfs(diameter: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         if root.is_none() {
//             return 0;
//         }

//         let root_node = root.unwrap();
//         let node = root_node.borrow();
//         let left_height: i32 = Solution::dfs(diameter, node.left.clone());
//         let right_height: i32 = Solution::dfs(diameter, node.right.clone());


//         if *diameter < left_height + right_height {
//             *diameter = left_height + right_height;
//         }

//         left_height.max(right_height) + 1
//     }
// }