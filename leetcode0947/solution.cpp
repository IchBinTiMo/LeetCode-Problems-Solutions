/*
Solution: Union Find

Time: O(n ^ 2) | Space: O(n)

Runtime: 80 ms | 26.78%
Memory: 20.68 MB | 67.2%

- n: length of 'stones'
*/

class Solution {
public:
    int removeStones(vector<vector<int>>& stones) {
        int n = stones.size();

        vector<vector<int>> adj(n, vector<int>());

        for (int i = 0; i < n; ++i) {
            for (int j = i + 1; j < n; ++j) {
                if (stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1]) {
                    adj[i].push_back(j);
                    adj[j].push_back(i);
                }
            }
        }

        vector<int> parents(n, -1);

        for (int i = 0; i < n; ++i) {
            if (parents[i] == -1) {
                union_find(&parents, i, i, &adj);
            }
        }

        int res = 0;

        for (auto &tmp: parents) {
            if (tmp != -1) {
                res += 1;
            }
        }

        return res;
    }

    void union_find(vector<int> *parents, int root, int current, vector<vector<int>> *adj) {
        for (auto &neighbor: (*adj)[current]) {
            if ((*parents)[neighbor] == -1 && neighbor != root) {
                (*parents)[neighbor] = root;
                union_find(parents, root, neighbor, adj);
            }
        }
    }
};