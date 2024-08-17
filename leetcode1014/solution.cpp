/*
Solution: DP

Time O(n) | Space O(1)

Runtime: 27 ms, 95.35%
Memory Usage: 43.38 MB, 59.04%

- n: length of 'values'
*/

class Solution {
public:
    int maxScoreSightseeingPair(vector<int>& values) {
        int n = values.size();

        int left = values[0] + 0;
        int res = INT_MIN;

        for (int i = 1; i < n; ++i) {
            res = max(res, values[i] - i + left);
            left = max(left, values[i] + i);
        }

        return res;
    }
};