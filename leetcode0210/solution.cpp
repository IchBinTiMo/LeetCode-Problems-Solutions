/*
Solution 1:

BFS

Time: O(V + E) | Space: O(V + E)

- V: numCourses
- E: the number of prerequisites
*/

class Solution {
public:
    vector<int> findOrder(int numCourses, vector<vector<int>>& prerequisites) {
        vector<set<int>> graph(numCourses, set<int> ());

        vector<int> res;

        for (auto &pre: prerequisites) {
            int after = pre[0];
            int before = pre[1];

            graph[before].insert(after);
        }

        int visited = 0;

        vector<int> in_degree(numCourses, 0);

        for (int i = 0; i < numCourses; ++i) {
            for (auto &w: graph[i]) {
                in_degree[w] += 1;
            }
        }

        deque<int> dq;

        for (int i = 0; i < numCourses; ++i) {
            if (in_degree[i] == 0) {
                dq.push_back(i);
            }
        }

        // topological sort in BFS
        while (!dq.empty()) {
            int root = dq[0];
            dq.pop_front();

            for (auto it = graph[root].begin(); it != graph[root].end(); ++it) {
                if (--in_degree[*it] == 0) {
                    dq.push_back(*it);
                }
            }

            res.push_back(root);
            visited += 1;
        }

        if (visited != numCourses) {
            return {};
        }

        return res;
    }
};