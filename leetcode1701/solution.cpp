class Solution {
public:
    double averageWaitingTime(vector<vector<int>>& customers) {
        /*
        Time: O(n) | Space: O(1)
        where n is the length of customers
        */
        double acc = 0;
        int current = 0;


        for (auto& c: customers) {
            if (current < c[0]) {
                current = c[0] + c[1];
                acc += c[1];
            } else {
                current += c[1];
                acc += current - c[0];
            }
        }

        return acc / customers.size();
    }
};