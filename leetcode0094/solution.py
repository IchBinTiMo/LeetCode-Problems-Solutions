# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        self.ans = []
        self.inorder(root)

        return self.ans[::-1]


    def inorder(self, node):
        if not node:
            return
        self.inorder(node.left)
        self.ans.insert(0, node.val)
        self.inorder(node.right)
        