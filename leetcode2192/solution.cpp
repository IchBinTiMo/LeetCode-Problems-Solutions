class Solution {
public:
    vector<vector<int>> getAncestors(int n, vector<vector<int>>& edges) {
        /* BFS
        *
        * Time: O(E + V) | Space: O(E + V)
        * E: Number of edges
        * V: Number of vertices */
        vector<vector<int>> adj(n);

        for (auto &edge: edges) {
            int from = edge[0];
            int to = edge[1];

            adj[to].push_back(from);
        }

        vector<vector<int>> res(n);

        // BFS
        for (int i = 0; i < n; ++i) {
            deque<int> dq;
            vector<bool> visited(n, false);

            dq.push_back(i);

            while (!dq.empty()) {
                int current = dq.front();

                if (!visited[current]) {
                    visited[current] = true;
                    if (current != i) {
                        res[i].push_back(current);
                    }

                    for (auto &node: adj[current]) {
                        dq.push_back(node);
                    }

                }

                dq.pop_front();
            }
        }

        // Sort
        for (int i = 0; i < n; ++i) {
            sort(res[i].begin(), res[i].end());
        }

        return res;


    }
};

// class Solution {
// public:
//     vector<vector<int>> getAncestors(int n, vector<vector<int>>& edges) {
//         vector<vector<int>> adj(n);

//         for (auto &edge: edges) {
//             int from = edge[0];
//             int to = edge[1];

//             adj[to].push_back(from);
//         }

//         vector<set<int>> ancs(n);

//         for (int i = 0; i < n; ++i) {
//             deque<int> dq;
//             vector<bool> visited(n, false);

//             dq.push_back(i);

//             while (!dq.empty()) {
//                 int current = dq.front();
//                 visited[current] = true;

//                 for (auto &node: adj[current]) {
//                     if (!visited[node]) {
//                         dq.push_back(node);
//                         ancs[i].insert(node);
//                     }
//                 }

//                 dq.pop_front();
//             }
//         }

//         vector<vector<int>> res(n);

//         for (int i = 0; i < n; ++i) {
//             vector<int> vec(ancs[i].begin(), ancs[i].end());
//             res[i] = vec;
//         }

//         return res;


//     }
// };