# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        leaves1 = self.dfs([], root1)
        leaves2 = self.dfs([], root2)

        return leaves1 == leaves2
        
    def dfs(self, leaves, root):
        if root:
            leaves = self.dfs(leaves, root.left)
            leaves = self.dfs(leaves, root.right)

            if not root.left and not root.right:
                leaves.insert(0, root.val)

        return leaves