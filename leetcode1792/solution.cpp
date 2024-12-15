/*
Solution: Priority Queue

Time: O(m + n log(n)) | Space: O(n)

Runtime: 287 ms | 90.60%
Memory: 99.24 MB | 38.35%
*/

class Solution {
public:
    double maxAverageRatio(vector<vector<int>>& classes, int extraStudents) {
        priority_queue<pair<double, int>> pq;
        vector<int> extra (classes.size(), 0);

        for (int i = 0; i < classes.size(); ++i) {

            double pass = classes[i][0];
            double total = classes[i][1];

            double org = pass / total;
            double added = (pass + 1) / (total + 1);

            pq.push(make_pair(added - org, i));
        }

        double res = 0.0;

        while (extraStudents > 0) {

            int i = pq.top().second;

            extra[i] += 1;

            double pass = classes[i][0] + extra[i];
            double total = classes[i][1] + extra[i];

            double org = pass / total;
            double added = (pass + 1) / (total + 1);

            pq.pop();
            pq.push(make_pair(added - org, i));

            --extraStudents;
        }

        while (!pq.empty()) {
            int i = pq.top().second;

            double pass = classes[i][0] + extra[i];
            double total = classes[i][1] + extra[i];

            res += (pass / total);

            pq.pop();
        }

        res /= classes.size();

        return res;
    }
};