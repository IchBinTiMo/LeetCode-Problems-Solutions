/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 235 ms | 6.64%
Memory: 68.88 MB | 5.01%

- n: length of 'nums'
*/

class Solution {
public:
    int maxWidthRamp(vector<int>& nums) {
        int n = nums.size();

        int res = 0;

        map<int, int> first;

        for (int i = 0; i < n; ++i) {
            if (first.find(nums[i]) == first.end()) {
                first[nums[i]] = i;
            }
        }

        for (auto it = ++first.begin(); it != first.end(); ++it) {
            auto prev = it;
            --prev;

            it->second = min(it->second, prev->second);
        }

        for (int i = n - 1; i >= 0; --i) {
            if (i >= first[nums[i]]) {
                res = max(res, i - first[nums[i]]);
            }
        }

        return res;
    }
};