/*
Solution 1:

DP

Time: O(n) | Space: O(1)
*/

class Solution {
public:
    int climbStairs(int n) {
        int a = 1;
        int b = 1;

        for (int i = 2; i <= n; ++i) {
            int tmp = a + b;
            a = b;
            b = tmp;
        }

        return b;
    }
};