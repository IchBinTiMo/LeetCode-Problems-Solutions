/*
Solution 1: Sliding window

Time: O(nlogn) | Space: O(n)


Runtime: 279 ms, 5.66%
Memory Usage: 118.56 MB, 20.28%

- n: length of arrays
*/

class Solution {
public:
    int maxDistance(vector<vector<int>>& arrays) {
        vector<pair<int, int>> v;

        for (int i = 0; i < arrays.size(); ++i) {
            auto &arr = arrays[i];
            v.push_back({arr.front(), i});
            v.push_back({arr.back(), i});
        }

        sort(v.begin(), v.end());

        int res = INT_MIN;

        int n = v.size();

        int left = 0;
        int right = n - 1;

        while (left < right) {
            if (v[left].second == v[right].second) {
                if (v[left + 1].first - v[left].first < v[right].first - v[right - 1].first) {
                    left += 1;
                } else {
                    right -= 1;
                }

                continue;
            } else {
                res = max(res, v[right].first - v[left].first);
                break;
            }
        }

        return res;
    }
};