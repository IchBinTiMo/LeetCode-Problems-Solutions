class Solution {
public:
    vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
        /*
        Time O(n) | Space O(1)

        where n is the number of intervals
        */
        if (intervals.size() == 0) {
            // corner case
            // if 'intervals' is empty
            intervals.push_back(newInterval);
            return intervals;
        }

        int i = 0;
        int current = 0;
        bool done = false;

        vector<vector<int>> res;

        vector<int> to_push;

        while (i < intervals.size()) {
            if (current == 0) {
                // the first element in 'newInterval'
                if (newInterval[current] > intervals[i][1]) {
                    // if the interval ends before the new interval starts
                    res.push_back(intervals[i]);
                    i += 1;
                } else {
                    // if the start of interval overlaps with the new interval
                    to_push.push_back(min(intervals[i][0], newInterval[current]));
                    current += 1;
                }
            } else if (current == 1) {
                // the second element in 'newInterval'
                if (newInterval[current] > intervals[i][1]) {
                    // if the interval ends before the new interval ends
                    i += 1;
                    if (i == intervals.size() && to_push.size() == 1) {
                        // if we are in the last element in 'intervals'
                        // but we have only one element in 'to_push'
                        to_push.push_back(newInterval[current]);
                        done = true;
                    }
                } else if (newInterval[current] <= intervals[i][1] && newInterval[current] >= intervals[i][0]) {
                    // if the end of 'newInterval' overlaps with the current interval
                    to_push.push_back(intervals[i][1]);
                    i += 1;
                } else {
                    // if the end of 'newInterval' does not overlap with the current interval
                    // which means 'to_push' should be closed in the previous interval
                    to_push.push_back(newInterval[current]);
                }

                if (to_push.size() == 2) {
                    // if 'to_push' is closed
                    res.push_back(to_push);
                    current += 1;
                    done = true;
                }
            } else {
                // just simply push all remained intervals in 'intervals' to 'res'
                res.push_back(intervals[i]);
                i += 1;
            }
        }

        if (!done) {
            // corner case
            // the 'newInterval' does not overlap with any interval
            // and its start is greater than the end of the last interval in 'intervals'
            res.push_back(newInterval);
        }

        return res;
    }
};