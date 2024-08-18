/*
Solution 1: brute force

Time: O(n) | Space: O(n)

Runtime: 344 ms, faster than 5.05%
Memory Usage: 93.01 MB, less than 5.10%
*/

class Solution {
public:
    int nthUglyNumber(int n) {
        set<int> uglys;

        uglys.insert(1);

        bool changed = true;
        auto first = uglys.begin();

        int prime[3] = {2, 3, 5};

        while (changed) {
            changed = false;

            for (int i = 0; i < 3; ++i) {
                if (INT_MAX / *first < prime[i]) {
                    continue;
                }
                uglys.insert(*first * prime[i]);
                changed = true;
            }

            ++first;

        }
        vector<int> tmp(uglys.begin(), uglys.end());

        return tmp[n - 1];
    }
};