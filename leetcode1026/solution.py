# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxAncestorDiff(self, root: Optional[TreeNode]) -> int:
        return self.traversal(root, root.val, root.val, 0)


    def traversal(self, root, low, high, maximum):
        if root.val < low:
            low = root.val
        if root.val > high:
            high = root.val
        if high - low > maximum:
            maximum = high - low

        if root.left :
            maximum = self.traversal(root.left, low, high, maximum)

        if root.right:
            maximum = self.traversal(root.right, low, high, maximum)

        return maximum
    