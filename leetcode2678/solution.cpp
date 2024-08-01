/*
Solution 1:

Time: O(n) | Space: O(1)

- n: length of 'details'
*/

class Solution {
public:
    int countSeniors(vector<string>& details) {
        int res = 0;

        for (string d: details) {
            int s = (d[11] - '0') * 10 + d[12] - '0';
            res += s > 60;
        }

        return res;
    }
};