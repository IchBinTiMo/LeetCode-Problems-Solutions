/*
Solution 1: DP

Time: O(n) | Space: O(n)

Runtime: 0344 ms, faster than 100.00%
Memory Usage: 9.69 MB, less than 72.38%

- n: length of 'n'
*/

class Solution {
public:
    int nthUglyNumber(int n) {
        vector<int> dp(n + 1, 0);

        dp[1] = 1;

        int two = 1;
        int three = 1;
        int five = 1;

        int index = 2;

        while (index <= n) {
            int two_v = dp[two] * 2;
            int three_v = dp[three] * 3;
            int five_v = dp[five] * 5;

            int mini = min({two_v, three_v, five_v});

            dp[index] = mini;

            if (mini == two_v) {
                two += 1;
            }
            if (mini == three_v) {
                three += 1;
            }
            if (mini == five_v) {
                five += 1;
            }

            index += 1;
        }

        return dp[n];
    }
};

/*
Solution 2: brute force

Time: O(n) | Space: O(n)

Runtime: 344 ms, faster than 5.05%
Memory Usage: 93.01 MB, less than 5.10%

- n: length of 'n'
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