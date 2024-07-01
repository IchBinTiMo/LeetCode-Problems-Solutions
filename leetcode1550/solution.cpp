class Solution {
public:
    bool threeConsecutiveOdds(vector<int>& arr) {
        // Time: O(N) | Space: O(1)
        int count = 0;

        for (int& v: arr) {
            if ((v & 1) == 0) {
                count = 0;
            } else {
                count += 1;
                if (count > 2) {
                    return true;
                }
            }
        }

        return false;
    }
};