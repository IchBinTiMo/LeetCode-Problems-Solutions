/*
Solution:

Time: O(m * n) | Space: O(m * n)

Runtime: 6 ms | 90.55%
Memory: 134.02 MB | 82.62%

- m: Length of mat
- n: Length of mat[0]
*/

class Solution {
public:
    int firstCompleteIndex(vector<int>& arr, vector<vector<int>>& mat) {
        int m = mat.size();
        int n = mat[0].size();

        vector<pair<int, int>> pos (m * n + 1);

        vector<int> row_visited = vector(m, 0);
        vector<int> col_visited = vector(n, 0);

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                pos[mat[i][j]] = make_pair(i, j);
            }
        }

        for (int i = 0; i < arr.size(); ++i) {
            int num = arr[i];

            auto& [r, c] = pos[num];

            row_visited[r] += 1;
            col_visited[c] += 1;

            if (row_visited[r] == n || col_visited[c] == m) {
                return i;
            }
        }

        return 0;
    }
};