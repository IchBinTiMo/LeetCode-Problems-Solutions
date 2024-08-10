/*
Solution 1: DFS

Time: O(n) | Space: O(n ^ 2)

- n: length of grid
*/

class Solution {
public:
    int n;
    vector<vector<char>> grids;
    vector<vector<vector<bool>>> visited;

    int regionsBySlashes(vector<string>& grid) {
        n = grid.size();

        grids = vector<vector<char>>(n, vector<char>(n));

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < grid[0].size(); ++j) {
                grids[i][j] = grid[i][j];
            }
        }

        int res = 0;

        // 0: ' ', (1, 2): '/', (3, 4): '\'
        visited = vector<vector<vector<bool>>>(n, vector<vector<bool>>(n, vector<bool>(5, false)));


        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < n; ++j) {
                vector<vector<int>> q;

                if (grids[i][j] == ' ') {
                    if (!visited[i][j][0]) {
                        q.push_back(vector<int>({i, j, 0}));
                        dfs(i, j, 0);
                        res += 1;
                    }
                } else if (grids[i][j] == '/') {
                    if (!visited[i][j][1]) {
                        q.push_back(vector<int>({i, j, 1}));
                        dfs(i, j, 1);
                        res += 1;
                    }

                    if (!visited[i][j][2]) {
                        q.push_back(vector<int>({i, j, 2}));
                        dfs(i, j, 2);
                        res += 1;
                    }
                } else {
                    if (!visited[i][j][3]) {
                        q.push_back(vector<int>({i, j, 3}));
                        dfs(i, j, 3);
                        res += 1;
                    }

                    if (!visited[i][j][4]) {
                        q.push_back(vector<int>({i, j, 4}));
                        dfs(i, j, 4);
                        res += 1;
                    }

                }
            }
        }

        return res;
    }

    void dfs(int r, int c, int p) {
        visited[r][c][p] = true;

        if (p == 0) {
            if (c + 1 < n) {
                go_right(r, c);
            }

            if (c - 1 >= 0) {
                go_left(r, c);
            }

            if (r + 1 < n) {
                go_down(r, c);
            }

            if (r - 1 >= 0) {
                go_up(r, c);
            }
        } else if (p == 1) {
            if (c - 1 >= 0) {
                go_left(r, c);
            }

            if (r - 1 >= 0) {
                go_up(r, c);
            }
        } else if (p == 2) {
            if (c + 1 < n) {
                go_right(r, c);
            }

            if (r + 1 < n) {
                go_down(r, c);
            }
        } else if (p == 3) {
            if (c - 1 >= 0) {
                go_left(r, c);
            }

            if (r + 1 < n) {
                go_down(r, c);
            }
        } else if (p == 4) {
            if (c + 1 < n) {
                go_right(r, c);
            }

            if (r - 1 >= 0) {
                go_up(r, c);
            }
        }
    }

    void go_right(int r, int c) {
        if (grids[r][c + 1] == ' ' && !visited[r][c + 1][0]) {
            dfs(r, c + 1, 0);
        } else if (grids[r][c + 1] == '/' && !visited[r][c + 1][1]) {
            dfs(r, c + 1, 1);
        } else if (grids[r][c + 1] == '\\' && !visited[r][c + 1][3]) {
            dfs(r, c + 1, 3);
        }
    }

    void go_down(int r, int c) {
        if (grids[r + 1][c] == ' ' && !visited[r + 1][c][0]) {
            dfs(r + 1, c, 0);
        } else if (grids[r + 1][c] == '/' && !visited[r + 1][c][1]) {
            dfs(r + 1, c, 1);
        } else if (grids[r + 1][c] == '\\' && !visited[r + 1][c][4]) {
            dfs(r + 1, c, 4);
        }
    }

    void go_left(int r, int c) {
        if (grids[r][c - 1] == ' ' && !visited[r][c - 1][0]) {
            dfs(r, c - 1, 0);
        } else if (grids[r][c - 1] == '/' && !visited[r][c - 1][2]) {
            dfs(r, c - 1, 2);
        } else if (grids[r][c - 1] == '\\' && !visited[r][c - 1][4]) {
            dfs(r, c - 1, 4);
        }
    }

    void go_up(int r, int c) {
        if (grids[r - 1][c] == ' ' && !visited[r - 1][c][0]) {
            dfs(r - 1, c, 0);
        } else if (grids[r - 1][c] == '/' && !visited[r - 1][c][2]) {
            dfs(r - 1, c, 2);
        } else if (grids[r - 1][c] == '\\' && !visited[r - 1][c][3]) {
            dfs(r - 1, c, 3);
        }
    }
};