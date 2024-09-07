/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
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
/*
Solution: Two pointers

Time: O(n * m) | Space: O(n)

Runtime: 21ms | 80.54%
Memory: 30.80MB | 75.68%

- n: the number of nodes in the tree
- m: the number of nodes in the list
*/

class Solution {
public:
    bool isSubPath(ListNode* head, TreeNode* root) {
        bool res = false;
        ListNode* current = head;

        dfs(&res, current, head, root);

        return res;

    }

    void dfs(bool *res, ListNode* current, ListNode* head, TreeNode* root) {
        if (current == NULL) {
            *res = true;
        } else {
            if (current && root) {
                if (current->val == root->val) {
                    dfs(res, current->next, head, root->left);
                    dfs(res, current->next, head, root->right);
                } else if (head->val == root->val) {
                    dfs(res, current, head->next, root->left);
                    dfs(res, current, head->next, root->right);
                } else {
                    dfs(res, head, head, root->left);
                    dfs(res, head, head, root->right);
                }
            }
        }
    }
};