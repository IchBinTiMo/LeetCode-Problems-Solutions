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

- n: the number of nodes in the tree
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

/*
Solution 2: BFS

Time: O(n) | Space: O(n)

Runtime: 47 ms | 95.32%
Memory: 328.92 MB | 23.16%

- n: the number of nodes in the tree
*/
class Solution {
public:
    TreeNode* replaceValueInTree(TreeNode* root) {
        bfs(root);

        root->val = 0;

        return root;
    }

    void bfs(TreeNode* root) {
        vector<TreeNode*> prev;

        prev.push_back(root);

        while (!prev.empty()) {
            vector<TreeNode*> next;

            int sum = 0;

            for (auto &node: prev) {
                sum += node->left == NULL ? 0 : node->left->val;
                sum += node->right == NULL ? 0 : node->right->val;
            }

            while (!prev.empty()) {
                TreeNode* node = prev.back();

                prev.pop_back();

                int left = node->left == NULL ? 0 : node->left->val;
                int right = node->right == NULL ? 0 : node->right->val;

                if (node->left) {
                    node->left->val = sum - left - right;
                    next.push_back(node->left);
                }

                if (node->right) {
                    node->right->val = sum - left - right;
                    next.push_back(node->right);
                }

            }
            prev = next;
        }
    }
};