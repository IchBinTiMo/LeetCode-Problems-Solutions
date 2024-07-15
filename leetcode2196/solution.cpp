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
    TreeNode* createBinaryTree(vector<vector<int>>& descriptions) {
        /*
        Time: O(n) | Space: O(n)

        where n is the number of nodes in the tree
        */
        unordered_map<int, pair<int, int>> nodes;
        unordered_set<int> has_parents;

        for (auto& des: descriptions) {
            // build the tree as a kinda adjacency list
            if (nodes.find(des[0]) != NULL) {
                if (des[2]) {
                    nodes[des[0]].first = des[1];
                } else {
                    nodes[des[0]].second = des[1];
                }
            } else {
                if (des[2]) {
                    nodes.insert({des[0], {des[1], -1}});
                } else {
                    nodes.insert({des[0], {-1, des[1]}});
                }
            }
            has_parents.insert(des[1]);

        }
        TreeNode* root;

        for (auto& node: nodes) {
            if (has_parents.find(node.first) == NULL) {
                // found the root
                root = build(root, node.first, &nodes);
                break;
            }
        }

        return root;
    }

    TreeNode* build(TreeNode* root, int val, unordered_map<int, pair<int, int>> *nodes) {
        // build the tree
        if (val == -1) {
            return NULL;
        }
        root = new TreeNode(val);
        
        if ((*nodes)[val].first) {
            root->left = build(root->left, (*nodes)[val].first, nodes);
        }

        if ((*nodes)[val].second) {
            root->right = build(root->right, (*nodes)[val].second, nodes);
        }

        return root;
    }
};