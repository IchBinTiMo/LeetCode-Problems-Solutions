/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    bool hasPathSum(TreeNode* root, int targetSum) {
        /// DFS
        ///
        /// Time: O(n) | Space: O(1)
        if (root == NULL) return false;

        return helper(root, targetSum);
    }

    bool helper(TreeNode* root, int targetSum) {
        if (root->right == NULL && root->left == NULL) {
            if (targetSum - root->val == 0) {
                return true;
            } else {
                return false;
            }
        }


        return hasPathSum(root->left, targetSum - root->val) || hasPathSum(root->right, targetSum - root->val);
    }
};