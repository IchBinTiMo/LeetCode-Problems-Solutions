# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        leaves = self.traversal([], root1)
        leaves = self.match(leaves, root2)        
        
        return True if len(leaves) == 0 else False
    def traversal(self, leaves, root):
        if root.left:
            leaves = self.traversal(leaves, root.left)
        
        if root.right:
            leaves = self.traversal(leaves, root.right)

        if not root.left and not root.right:
            leaves.append(root.val)

        return leaves

    def match(self, leaves, root):
        if root.left:
            leaves = self.match(leaves, root.left)
        
        if root.right:
            leaves = self.match(leaves, root.right)

        if not root.left and not root.right:
            if len(leaves) and root.val == leaves[0]:
                leaves.pop(0)
            else:
                leaves.append("bad")

        return leaves
            

# # Definition for a binary tree node.
# # class TreeNode:
# #     def __init__(self, val=0, left=None, right=None):
# #         self.val = val
# #         self.left = left
# #         self.right = right
# class Solution:
#     def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
#         leaves1 = self.dfs([], root1)
#         leaves2 = self.dfs([], root2)

#         return leaves1 == leaves2
        
#     def dfs(self, leaves, root):
#         if root:
#             leaves = self.dfs(leaves, root.left)
#             leaves = self.dfs(leaves, root.right)

#             if not root.left and not root.right:
#                 leaves.insert(0, root.val)

#         return leaves