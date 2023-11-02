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
use std::collections::HashMap;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inorder(tree: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) {
            if let Some(node) = tree {
                inorder(node.borrow_mut().left.clone(), map);
                *map.entry(node.borrow().val).or_insert(0) += 1;
                inorder(node.borrow_mut().right.clone(), map);
            }
            
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        inorder(root, &mut map);

        let max = map.values().max().unwrap();

        let ans: Vec<i32> = map.keys().filter(|x| match map.get(x) {
                Some(v) => *v == *max,
                None => false
            }).collect::<Vec<&i32>>().iter().map(|x| **x).collect::<Vec<i32>>();


        ans
    }

    
}