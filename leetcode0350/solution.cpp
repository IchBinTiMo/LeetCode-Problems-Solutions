class Solution {
public:
    vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
        /* 
        Time: O(N log N + M log M) | Space: O(N + M)

        N, M: Length of nums1, nums2

        Explanation:
        The reason why there are log in Time Complexity
        is because map sorts the keys in ascending order automatically.
        */
        map<int, int> count;

        for (int& num: nums1) {
            if (count.find(num) == count.end()) {
                count[num] = 1;
            } else {
                count[num] += 1;
            }
        }

        vector<int> res;

        for (int& num: nums2) {
            if (count.find(num) == count.end()) {
                continue;
            } else {
                if (count[num] > 0) {
                    count[num] -= 1;
                    res.push_back(num);
                }
            }
        }

        return res;
    }
};