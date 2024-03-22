// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // create a vector of the values
        let mut nums: Vec<i32> = Vec::new();

        let mut current: Option<Box<ListNode>> = head.clone();

        // push the values into the vector
        while let Some(node) = current {
            nums.push(node.val);
            current = node.next;
        }

        // check if the vector is a palindrome
        for i in 0..nums.len() {
            if nums[i] != nums[nums.len() - 1 - i] {
                return false;
            }
        }

        true
    }
}