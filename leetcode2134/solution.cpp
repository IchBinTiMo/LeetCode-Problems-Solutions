/*
Solution 1:

Sliding Window

Time: O(n) | Space: O(1)

- n: length of 'nums'
*/

class Solution {
public:
    int minSwaps(vector<int>& nums) {
        int n = nums.size();
        int ones = 0;

        for (int &num: nums) {
            ones += num;
        }

        int sum = 0;

        for (int i = 0; i < ones; ++i) {
            sum += nums[i];
        }

        int res = ones - sum;

        for (int i = 0; i < n; ++i) {
            sum += nums[(i + ones) % n] - nums[i];

            res = min(res, ones - sum);
        }

        return res;
    }
};