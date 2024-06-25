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
    /// DFS

    /// Time: O(n) | Space: O(1)
    /// where n is the number of nodes in the tree
public:
    TreeNode* bstToGst(TreeNode* root) {
        int acc = 0;
        traverse(&acc, root);

        return root;
    }

    // In-order traversal and traverse the right node first instead of left
    void traverse(int* acc, TreeNode* root) {
        if (root != NULL) {
            traverse(acc, root->right); // traverse right
            root->val += *acc; // update root
            *acc = root->val; // update acc
            traverse(acc, root->left); // traverse left
        }
    }
};