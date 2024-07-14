class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        /*
        Time: O(log(n)) | Space: O(1)

        where n is the length of nums
        */
        int left = 0;
        int right = nums.size() - 1;

        while (left <= right) {
            int mid = (left + right) / 2;

            if (nums[mid] < target) {
                left = mid + 1;
            } else if (nums[mid] > target) {
                right = mid - 1;
            } else {
                return mid;
            }
        }

        return left;
    }
};