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

Time: O(n + m) | Space: O(n)

Runtime: 492ms | 29.28%
Memory: 260.89MB | 62.09%

- n: length of 'nums'
- m: number of nodes in 'head'
*/
class Solution {
public:
    ListNode* modifiedList(vector<int>& nums, ListNode* head) {
        set<int> to_remove(nums.begin(), nums.end());

        ListNode* prev = NULL;
        ListNode* current = head;

        while (current != NULL) {
            if (to_remove.find(current->val) != to_remove.end()) {
                if (prev == NULL) {
                    current = current->next;
                    head = current;
                } else {
                    prev->next = current->next;
                    current = current->next;
                }
                continue;
            }
            
            prev = current;
            current = current->next;
        }

        return head;
    }
};