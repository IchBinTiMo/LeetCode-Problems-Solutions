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
    vector<int> nodesBetweenCriticalPoints(ListNode* head) {
        /*
        Time: O(n) | Space: O(1)

        where 'n' is the number of nodes in the given list.
        */
        int first = -1;
        int prev_point = -1;
        int mini = INT_MAX;
        int idx = 1;

        int prev = head->val;
        ListNode* current = head->next;

        while (current->next != NULL) {
            if ((current->val > prev && current->val > current->next->val) || (current->val < prev && current->val < current->next->val)) {
                if (first == -1) {
                    first = idx;
                } else {
                    mini = min(mini, idx - prev_point);
                }
                prev_point = idx;
            }

            prev = current->val;
            current = current->next;
            idx += 1;
        }

        vector<int> v;

        if (mini == INT_MAX) {
            v.push_back(-1);
            v.push_back(-1);
        } else {
            v.push_back(mini);
            v.push_back(prev_point - first);
        }

        return v;
    }
};