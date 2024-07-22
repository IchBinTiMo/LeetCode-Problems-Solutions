/*
Solution 1:

BFS, Topological Sort

Time: O(k + m + n) | Space: O(k + m + n)

- m: Length of rowConditions
- n: Length of colConditions
*/

class Solution {
public:
    vector<vector<int>> buildMatrix(int k, vector<vector<int>>& rowConditions, vector<vector<int>>& colConditions) {
        vector<vector<int>> res(k, vector<int> (k, 0));

        vector<set<int>> row_adj(k + 1, set<int>());
        vector<set<int>> col_adj(k + 1, set<int>());

        for (auto &r: rowConditions) {
            int above = r[0];
            int below = r[1];

            row_adj[above].insert(below);
        }

        for (auto &c: colConditions) {
            int left = c[0];
            int right = c[1];

            col_adj[left].insert(right);
        }

        vector<int> row_in(k + 1, 0);
        vector<int> col_in(k + 1, 0);

        for (int v = 1; v <= k; ++v) {
            for (auto &w: row_adj[v]) {
                row_in[w] += 1;
            }
        }
        
        for (int v = 1; v <= k; ++v) {
            for (auto &w: col_adj[v]) {
                col_in[w] += 1;
            }
        }

        vector<int> row;
        vector<int> col;

        deque<int> dq;

        for (int i = 1; i <= k; ++i) {
            if (row_in[i] == 0) {
                dq.push_back(i);
            }
        }

        // Topological Sort in BFS
        while (!dq.empty()) {
            int root = dq[0];
            dq.pop_front();
            row.push_back(root);

            for (auto it = row_adj[root].begin(); it != row_adj[root].end(); ++it) {
                if (--row_in[*it] == 0) {
                    dq.push_back(*it);
                }
            }
        }

        for (int i = 1; i <= k; ++i) {
            if (col_in[i] == 0) {
                dq.push_back(i);
            }
        }

        while (!dq.empty()) {
            int root = dq[0];
            dq.pop_front();
            col.push_back(root);

            for (auto it = col_adj[root].begin(); it != col_adj[root].end(); ++it) {
                if (--col_in[*it] == 0) {
                    dq.push_back(*it);
                }
            }
        }

        if (row.size() != k || col.size() != k) {
            return {};
        }

        vector<vector<int>> positions(k + 1, vector<int> (2, -1));

        for (int i = 0; i < row.size(); ++i) {
            positions[row[i]][0] = i;
        }

        for (int i = 0; i < col.size(); ++i) {
            positions[col[i]][1] = i;
        }

        for (int i = 1; i <= k; ++i) {
            int r = positions[i][0];
            int c = positions[i][1];
            res[r][c] = i;
        }

        return res;
    }
};
