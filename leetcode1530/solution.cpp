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
    int countPairs(TreeNode* root, int distance) {
        /*
        DFS

        Time O(n) | Space O(n)
        
        where n is the number of nodes in the tree
        */
        int res = 0;

        traverse(&res, root, distance);

        return res;
    }

    map<int, int> traverse(int* res, TreeNode* root, int distance) {
        if (root == NULL) {
            return map<int, int>();
        }

        map<int, int> left = traverse(res, root->left, distance);
        map<int, int> right = traverse(res, root->right, distance);

        map<int, int> ret;


        if (left.size() == 0 && right.size() == 0) {
            // if 'root' is a leaf
            ret.insert({1, 1});
        } else {

            for (auto l = left.begin(); l != left.end(); ++l) {
                for (auto r = right.begin(); r != right.end(); ++r) {
                    if (l->first + r->first <= distance) {
                        // update the number of pairs
                        *res += l->second * r->second;
                    }
                }
            }

            // merge the maps we got from the left and right subtrees
            // and all keys should be increased by 1
            // because we are gonna back to the parent of 'root'
            // and the depth will be increased by 1
            for (auto it = left.begin(); it != left.end(); ++it) {
                ret[it->first + 1] += it->second;
            }

            for (auto it = right.begin(); it != right.end(); ++it) {
                ret[it->first + 1] += it->second;
            }

        }

        return ret;
    }
};