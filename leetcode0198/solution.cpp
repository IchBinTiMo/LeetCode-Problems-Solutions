/*
Solution 1:

DP

Time: O(n) | Space: O(n)

- n: length of 'nums'
*/

class Solution {
public:
    int rob(vector<int>& nums) {
        int n = nums.size();
        vector<int> dp(n + 1, 0);
        dp[1] = nums[0];

        int res = 0;

        for (int i = 2; i <= n; ++i) {
            dp[i] = max(dp[i - 1], nums[i - 1] + dp[i - 2]);
        }

        return dp[n];
    }
};

/*
Solution 2:

DP

Time: O(n) | Space: O(1)

- n: length of 'nums'
*/
class Solution {
public:
    int rob(vector<int>& nums) {
        int n = nums.size();

        int a = 0;
        int b = nums[0];

        for (int i = 1; i < n; ++i) {
            int tmp = max(b, nums[i] + a);
            a = b;
            b = tmp;
        }

        return b;
    }
};