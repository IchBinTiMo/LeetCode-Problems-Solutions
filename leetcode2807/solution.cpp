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

Time: O(n) | Space: O(1)

Runtime: 42 ms | 45.06%
Memory: 35.51 MB | 49.64%

- n: the number of nodes in the linked list
*/
class Solution {
public:
    ListNode* insertGreatestCommonDivisors(ListNode* head) {
        ListNode* current = head;

        while (current->next) {
            ListNode* tmp = new ListNode(-1, current->next);
            
            if (current->val >= current->next->val) {
                tmp->val = findGCD(current->next->val, current->val);
            } else {
                tmp->val = findGCD(current->val, current->next->val);
            }
            
            current->next = tmp;
            current = tmp->next;
        }

        return head;
    }

    int findGCD(int a, int b) {
        if (b % a == 0) {
            return a;
        } else {
            return findGCD(b % a, a);
        }
    }
};