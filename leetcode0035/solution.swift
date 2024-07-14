class Solution {
    func searchInsert(_ nums: [Int], _ target: Int) -> Int {
        /*
        Time: O(log n) | Space: O(1)

        where n is the length of `nums`
        */
        var left = 0;
        var right = nums.count - 1;

        while (left <= right) {
            var mid = (left + right) / 2;

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
}