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
    vector<TreeNode*> delNodes(TreeNode* root, vector<int>& to_delete) {
        /*
        DFS

        Time O(N) | Space O(N)

        where N is the number of nodes in the tree
        */
        unordered_set<int> to_del(to_delete.begin(), to_delete.end());
        vector<TreeNode*> res;

        to_del.insert(-1);

        traverse(&res, -1, root, &to_del);

        return res;
    }

    void traverse(vector<TreeNode*> *res, int parent, TreeNode* current, unordered_set<int> *to_del) {
        if (current == NULL) {
            return;
        }

        traverse(res, current->val, current->left, to_del);
        traverse(res, current->val, current->right, to_del);

        if (current->left != NULL && to_del->count(current->left->val)) {
            current->left = NULL;
        }

        if (current->right != NULL && to_del->count(current->right->val)) {
            current->right = NULL;
        } 

        if (to_del->count(parent) && !to_del->count(current->val)) {
            res->push_back(current);
        }

    }
};