/*
Solution: DP

Time: O(n^2) | Space: O(1)

Runtime: 12 ms, faster than 60.16%
Memory Usage: 13.29 MB, less than 88.13%

- n: length of 'matrix'
*/

class Solution {
public:
    int minFallingPathSum(vector<vector<int>>& matrix) {
        int n = matrix.size();

        for (int i = 1; i < n; ++i) {
            for (int j = 0; j < n; ++j) {
                int mini = INT_MAX;
                
                if (j > 0 && j < n - 1) {
                    mini = min({mini, matrix[i - 1][j - 1], matrix[i - 1][j], matrix[i - 1][j + 1]});
                } else if (j > 0) {
                    mini = min({mini, matrix[i - 1][j - 1], matrix[i - 1][j]});
                } else {
                    mini = min({mini, matrix[i - 1][j + 1], matrix[i - 1][j]});
                }

                matrix[i][j] += mini;
            }
        }

        int res = INT_MAX;

        for (int i = 0; i < n; ++i) {
            res = min(res, matrix[n - 1][i]);
        }

        return res;
    }
};