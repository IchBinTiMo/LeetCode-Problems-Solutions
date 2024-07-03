class Solution {
public:
    int minDifference(vector<int>& nums) {
        /*
        Time: O(n log n) | Space: O(1)

        n: length of nums
        */
        if (nums.size() <= 4) {
            return 0;
        }

        sort(nums.begin(), nums.end());

        int res = INT_MAX;

        for (int i = 0; i < 4; i++) {
            res = min(res, nums[i + nums.size() - 4] - nums[i]);
        }

        return res;
    }
};