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

        priority_queue<tuple<int, int, int>> pq; // this is why there is a V log V in Time Complexity

        for (auto& edge: edges) {
            int type = edge[0];
            int from = edge[1];
            int to = edge[2];

            pq.push({type, from - 1, to - 1});
        }

        int to_remove = 0;

        while (!pq.empty()) {
            auto [type, x, y] = pq.top();

            int px_a = find(parent_a, x); // parent of x in Alice's tree
            int py_a = find(parent_a, y); // parent of y in Alice's tree
            int px_b = find(parent_b, x); // parent of x in Bob's tree
            int py_b = find(parent_b, y); // parent of y in Bob's tree

            if (type == 3) {
                if (px_a == py_a && px_b == py_b) {
                    to_remove += 1;
                } else {
                    if (px_a != py_a) {
                        unionn(parent_a, size_a, px_a, py_a);
                    }
                    if (px_b != py_b) {
                        unionn(parent_b, size_b, px_b, py_b);
                    }
                }
            } else if (type == 2) {
                if (px_b == py_b) {
                    to_remove += 1;
                } else {
                    unionn(parent_b, size_b, px_b, py_b);
                }
            } else {
                if (px_a == py_a) {
                    to_remove += 1;
                } else {
                    unionn(parent_a, size_a, px_a, py_a);
                }
            }

            pq.pop();
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

    void unionn(vector<int>& parent, vector<int>& size, int x, int y) {
        if (size[x] < size[y]) {
            parent[x] = y;
            size[y] += size[x];
        } else {
            parent[y] = x;
            size[x] += size[y];
        }
    }
};