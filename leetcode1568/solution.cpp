/*
Solution 1: DFS + Brute Force

Time: O(m * n) | Space: O(m * n)

- m: Length of grid
- n: Length of grid[0]
*/

class Solution {
public:
    int m;
    int n;
    vector<vector<int>> grids;
    vector<vector<bool>> visited;
    int minDays(vector<vector<int>>& grid) {
        m = grid.size();
        n = grid[0].size();
        grids = grid;

        if (count() != 1) {
            return 0;
        }

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grids[i][j]) {
                    grids[i][j] = 0;
                    if (count() != 1) {
                        return 1;
                    }
                    grids[i][j] = 1;
                }
            }
        }
        
        return 2;
    }

    int count() {
        int cnt = 0;
        visited = vector<vector<bool>>(m, vector<bool>(n, false));
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grids[i][j] && !visited[i][j]) {
                    dfs(i, j);
                    cnt += 1;
                }
            }
        }

        return cnt;
    }

    void dfs(int r, int c) {
        visited[r][c] = true;

        if (r - 1 >= 0 && grids[r - 1][c] && !visited[r - 1][c]) {
            dfs(r - 1, c);
        }

        if (c + 1 < n && grids[r][c + 1] && !visited[r][c + 1]) {
            dfs(r, c + 1);
        }

        if (r + 1 < m && grids[r + 1][c] && !visited[r + 1][c]) {
            dfs(r + 1, c);
        }

        if (c - 1 >= 0 && grids[r][c - 1] && !visited[r][c - 1]) {
            dfs(r, c - 1);
        }
    }
};