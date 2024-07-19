class Solution {
public:
    vector<string> summaryRanges(vector<int>& nums) {
        /*
        Sliding window

        Time: O(n) | Space: O(n)

        where 'n' is the length of 'nums'
        */
        vector<string> res;

        int left = 0;
        int right = 0;

        while (left < nums.size() && right < nums.size()) {
            while (right + 1 < nums.size() && nums[right] + 1 == nums[right + 1]) {
                right += 1;
            }

            string to_push;

            if (left == right) {
                // if 'left' and 'right' are equal
                // which means there is only one number in the range
                to_push = to_string(nums[left]);
            } else {
                to_push = to_string(nums[left]) + "->" + to_string(nums[right]);
            }

            res.push_back(to_push);

            right += 1;
            left = right;
        }

        return res;
    }
};