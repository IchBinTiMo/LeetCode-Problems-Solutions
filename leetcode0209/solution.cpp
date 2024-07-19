class Solution {
public:
    int minSubArrayLen(int target, vector<int>& nums) {
        /*
        Sliding Window

        Time: O(n) | Space: O(1)

        where 'n' is the length of 'nums'
        */
        int res = INT_MAX;

        int left = 0;
        int right = 0;

        int sum = 0;

        while (right < nums.size()) {
            while (right < nums.size() && sum < target) {
                sum += nums[right];
                right += 1;
            }

            while (left < right && sum >= target) {
                res = min(res, right - left); // update the min subarray length
                sum -= nums[left];
                left += 1;
            }
        }



        if (res == INT_MAX) {
            return 0;
        } else {
            return res;
        }
    }
};

/*
target = 7
nums = [2,3,1,2,4,3]

sum = 0
left = 0, right = 0

sum = 2
left = 0, right = 1

sum = 5
left = 0, right = 2

sum = 6
left = 0, right = 3

sum = 8
left = 0, right = 4

res = 4
sum = 6
left = 1, right = 4

res = 4
sum = 6
left = 1, right = 4

sum = 10
left = 1, right = 5

res = 4
sum = 7
left = 2, right = 5

res = 3
sum = 6
left = 3, right = 5

sum = 9
left = 3, right = 6

res = 3
sum = 7
left = 4, right = 6

res = 2
sum = 3
left = 5, right = 6
*/