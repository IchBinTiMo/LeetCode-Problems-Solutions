class Solution {
public:
    int maxNumEdgesToRemove(int n, vector<vector<int>>& edges) {
        /* Union Find
        
        Time: O(E + V log V) | Space: O(E + V)
        E: Number of edges
        V: Number of vertices
        */

        vector<int> parent_a(n, -1), parent_b(n, -1);
        vector<int> size_a(n, 1), size_b(n, 1);

        set<tuple<int, int, int>> to_remove;

        priority_queue<tuple<int, int, int>> pq_a, pq_b; // this is why there is a V log V in Time Complexity

        for (auto& edge: edges) {
            int type = edge[0];
            int from = edge[1];
            int to = edge[2];

            if (type == 1) {
                pq_a.push({type, from - 1, to - 1});
            } else if (type == 2) {
                pq_b.push({type, from - 1, to - 1});
            } else {
                pq_a.push({type, from - 1, to - 1});
                pq_b.push({type, from - 1, to - 1});
            }
        }

        int remain_a = 0, remain_b = 0;

        while (!pq_a.empty()) {
            auto [type, from, to] = pq_a.top();

            int pf = find(parent_a, from); 
            int pt = find(parent_a, to);

            if (pf == pt) {
                to_remove.insert({type, from, to});
            } else {
                if (size_a[pf] < size_a[pt]) {
                    parent_a[pf] = pt;
                    size_a[pt] += size_a[pf];
                } else {
                    parent_a[pt] = pf;
                    size_a[pf] += size_a[pt];
                }
            }
            pq_a.pop();
        }

        while (!pq_b.empty()) {
            auto [type, from, to] = pq_b.top();

            int pf = find(parent_b, from); 
            int pt = find(parent_b, to);

            if (pf == pt) {
                to_remove.insert({type, from, to});
            } else {
                if (size_b[pf] < size_b[pt]) {
                    parent_b[pf] = pt;
                    size_b[pt] += size_b[pf];
                } else {
                    parent_b[pt] = pf;
                    size_b[pf] += size_b[pt];
                }
            }
            pq_b.pop();
        }

        int self_parent = 0;

        // to check if there is a vertex whose parent is -1
        for (int& p: parent_a) {
            if (p == -1) {
                self_parent += 1;
            }
        }

        for (int& p: parent_b) {
            if (p == -1) {
                self_parent += 1;
            }
        }

        if (self_parent > 2) {
            return -1;
        } else {
            return to_remove.size();
        }
    }

    int find(vector<int>& parent, int current) {
        // find the root
        if (parent[current] == -1) {
            return current;
        } else {
            return find(parent, parent[current]);
        }
    }
};