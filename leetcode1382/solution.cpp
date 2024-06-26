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
    TreeNode* balanceBST(TreeNode* root) {
        vector<TreeNode*> v;

        in_order(root, v);

        return build_tree(v, 0, v.size());
    }

    void in_order(TreeNode* root, vector<TreeNode*> &v) {
        if (root == NULL) return;
        in_order(root->left, v);
        v.push_back(root);
        in_order(root->right, v);
    }

    TreeNode* build_tree(vector<TreeNode*> &v, int left, int right) {
        if (left == right) return NULL;
        int mid = (left + right) / 2;
        TreeNode* root = v[mid];
        root->right = build_tree(v, mid + 1, right);
        root->left = build_tree(v, left, mid);

        return root;
    }
};