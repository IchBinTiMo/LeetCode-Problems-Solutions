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

/*
Solution:

Time: O(k) | Space: O(m * n)

Runtime: 162 ms | 65.87%
Memory: 130.56 MB | 65.45%

- k: the number of nodes in the linked list 'head'
*/
class Solution {
public:
    vector<vector<int>> spiralMatrix(int m, int n, ListNode* head) {
        ListNode* current = head;
        int r = 0;
        int c = 0;
        int direction = 0;
        int steps[5] = {0, 1, 0, -1, 0};

        vector<vector<int>> res(m, vector<int>(n, -1));

        while (current) {
            res[r][c] = current->val;
            current = current->next;
            int new_r = r + steps[direction];
            int new_c = c + steps[direction + 1];

            if (new_r >= m || new_r < 0 ||
                new_c >= n || new_c < 0 ||
                res[new_r][new_c] != -1) {
                    direction = (direction + 1) % 4;
                }

            r = r + steps[direction];
            c = c + steps[direction + 1];
        }

        return res;
    }
};