/*
Solution 1:

Floyd-Warshall Algorithm

Time: O(n ^ 3) | Space: O(n ^ 2)

- n: length of edges
*/

class Solution {
public:
    int findTheCity(int n, vector<vector<int>>& edges, int distanceThreshold) {
        vector<vector<long long>> dp(n, vector<long long>(n, INT_MAX));

        for (auto &edge: edges) {
            int from = edge[0];
            int to = edge[1];
            int weight = edge[2];

            // it is bidirectional
            dp[from][to] = (long long)weight;
            dp[to][from] = (long long)weight;
        }

        for (int k = 0; k < n; ++k) {
            dp[k][k] = 0; // the distance from k to k is 0
            for (int i = 0; i < n; ++i) {
                for (int j = 0; j < n; ++j) {
                    dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]); // update
                }
            }
        }

        int res = 0;
        int minimum = INT_MAX; // current minimum count

        for (int i = 0; i < n; ++i) {
            int count = 0; // count of reachable cities within distanceThreshold
            for (int j = 0; j < n; ++j) {
                if (dp[i][j] <= distanceThreshold) {
                    count += 1;
                }
            }

            if (count <= minimum) {
                // update
                res = i;
                minimum = count;
            }
        }

        return res;
    }
};