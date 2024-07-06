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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        /*
        Time: O(n) | Space: O(n)

        where 'n' is the number of nodes in the longer given list.
        */
        ListNode* res = new ListNode();
        ListNode* current = res;
        int flag = 0;

        while (true) {
            int val = flag;

            if (l1 != NULL) {
                val += l1->val;
                l1 = l1->next;
            }

            if (l2 != NULL) {
                val += l2->val;
                l2 = l2->next;
            }

            flag = val / 10;
            val %= 10;

            current->val = val;

            if (l1 == NULL && l2 == NULL && flag == 0) {
                break;
            }
            current->next = new ListNode();
            current = current->next;


        }

        return res;
    }
};