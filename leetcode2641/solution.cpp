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

/*
Solution 1: DFS

Time: O(n) | Space: O(n)

Runtime: 10 ms | 98.66%
Memory: 314.59 MB | 28.95%
*/
class Solution {
public:
    vector<int> sums;
    TreeNode* replaceValueInTree(TreeNode* root) {
        traverse(0, root);

        replace(0, root);

        return root;
    }

    void traverse(int current, TreeNode* root) {
        if (!root) return;
        if (current == sums.size()) {
            sums.push_back(root->val);
        } else {
            sums[current] += root->val;
        }

        traverse(current + 1, root->left);
        traverse(current + 1, root->right);

        int left = root->left == NULL ? 0 : root->left->val;
        int right = root->right == NULL ? 0 : root->right->val;

        if (root->left != NULL) {
            root->left->val = left + right;
        }

        if (root->right != NULL) {
            root->right->val = left + right;
        }
        
    }

    void replace(int current, TreeNode* root) {
        if (!root) return;

        root->val = sums[current] - root->val;
        
        replace(current + 1, root->left);
        replace(current + 1, root->right);
        
    }
};