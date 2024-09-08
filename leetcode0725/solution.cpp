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

Time: O(n) | Space: O(n)

Runtime: 4ms | 73.95%
Memory: 13.88MB | 61.52%
*/

class Solution {
public:
    vector<ListNode*> splitListToParts(ListNode* head, int k) {
        ListNode* current = head;
        int cnt = 0;

        while (current) {
            cnt += 1;
            current = current->next;
        }

        current = head;

        vector<ListNode*> res;
        int mod = cnt % k;

        for (int i = 0; i < k; ++i) {
            ListNode* prev = current;
            
            int len = cnt / k + (mod > 0);
            mod -= 1;

            for (int j = 0; j < len - 1; ++j) {
                current = current->next;
            }

            if (current) {
                ListNode* tail = current;
                current = current->next;
                tail->next = NULL;
            }

            res.push_back(prev);
        }

        return res;
    }
};