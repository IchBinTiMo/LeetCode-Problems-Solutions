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
class Solution {
public:
    ListNode* mergeNodes(ListNode* head) {
        /*
        Time: O(n) | Space: O(1)

        where 'n' is the number of nodes in the given list.
        */
        ListNode* left = head;
        ListNode* right = head->next;

        while (left != NULL && right != NULL) {
            if (right->val == 0) {
                if (right->next == NULL) {
                    left->next = NULL;
                    break;
                }
                left->next = right;
                left = left->next;
            } else {
                left->val += right->val;
            }
            right = right->next;
        }


        return head;
    }
};