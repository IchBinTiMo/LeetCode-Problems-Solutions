/*
Solution: BFS

Time: O(n) | Space: O(n)

Runtime: 4 ms | 42.65%
Memory: 89.89 MB | 5.07%

- n: the number of nodes in the tree
*/

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
    TreeNode* reverseOddLevels(TreeNode* root) {
        vector<TreeNode*> q;
        int level = 0;

        q.push_back(root);

        while (q.size()) {
            vector<TreeNode*> next;

            if (q[0]) {
                if (level & 1) {
                    int n = q.size();
                    for (int i = 0; i < n / 2; ++i) {
                        q[i]->val ^= q[n - i - 1]->val;
                        q[n - i - 1]->val ^= q[i]->val;
                        q[i]->val ^= q[n - i - 1]->val;
                    }
                }

                for (auto &node: q) {
                    next.push_back(node->left);
                    next.push_back(node->right);
                }
            }

            q = next;
            level += 1;
        }

        return root;
    }
};