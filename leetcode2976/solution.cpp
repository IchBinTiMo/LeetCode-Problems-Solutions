/*
Solution 1:

Floyd-Warshall Algorithm

Time: O(n + C) | Space: O(n ^ 2)

- n: length of edges
- C: the maximum value of C is (26 ^ 3 + 25 ^ 2)
*/

class Solution {
public:
    long long minimumCost(string source, string target, vector<char>& original, vector<char>& changed, vector<int>& cost) {
        vector<vector<int>> costs (26, vector<int> (26, -1));

        // the cost is 0 if the letter doesn't change
        for (int i = 0; i < 26; ++i) {
            costs[i][i] = 0;
        }

        // initialize costs
        for (int i = 0; i < cost.size(); ++i) {
            int from = original[i] - 'a';
            int to = changed[i] - 'a';

            if (costs[from][to] == -1) {
                costs[from][to] = cost[i];
            } else {
                costs[from][to] = min(costs[from][to], cost[i]);
            }

        }

        // floyd-warshall
        for (int k = 0; k < 26; ++k) {
            for (int i = 0; i < 26; ++i) {
                for (int j = 0; j < 26; ++j) {
                    if (costs[i][k] == -1 || costs[k][j] == -1) {
                        continue;
                    }

                    if (costs[i][j] == -1) {
                        costs[i][j] = costs[i][k] + costs[k][j];
                    } else {
                        costs[i][j] = min(costs[i][j], costs[i][k] + costs[k][j]);
                    }
                }
            }
        }

        long long res = 0;

        for (int i = 0; i < source.size(); ++i) {
            int from = source[i] - 'a';
            int to = target[i] - 'a';

            if (costs[from][to] == -1) {
                return -1;
            }

            res += costs[from][to];
        }
        
        return res;
    }
};