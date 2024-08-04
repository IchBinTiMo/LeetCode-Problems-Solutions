/*
Solution 1: Binary Search

Time: O(n log n) | Space: O(n)

- n: length of nums
*/

class Solution {
public:
    int lengthOfLIS(vector<int>& nums) {
        if (nums.size() == 1) return 1;

        vector<int> v;
        v.push_back(nums[0]);

        for (int i = 1; i < nums.size(); ++i) {
            if (nums[i] > v.back()) {
                v.push_back(nums[i]);
            } else {
                int idx = find_idx(v, 0, v.size() - 1, nums[i]);
                v[idx] = nums[i];
            }
        }

        return v.size();
    }

    int find_idx(vector<int>& nums, int left, int right, int target) {
        while (left < right) {
            int mid = (left + right) / 2;

            if (right - left < 1) {
                return left;
            } else if (target <= nums[mid]) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return left;
    }
};

/*
Solution 2: Binary Search

Time: O(n log n) | Space: O(n)

- n: length of nums
*/

class Solution {
public:
    int lengthOfLIS(vector<int>& nums) {
        if (nums.size() == 1) return 1;

        vector<int> v;
        v.push_back(nums[0]);

        for (int i = 1; i < nums.size(); ++i) {
            int n = nums[i];

            if (n > v.back()) {
                v.push_back(n);
            }
            else {
                *lower_bound(v.begin(), v.end(), n) = n;
            }
            
        }

        return v.size();
    }
};