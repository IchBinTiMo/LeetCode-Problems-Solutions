/*
Solution 1:

Quick sort

Time: O(n log n) | Space: O(log n)

- n: length of nums
*/

class Solution {
public:
    vector<int> sortArray(vector<int>& nums) {
        quick_sort(nums, 0, nums.size() - 1);
        return nums;
    }

    void quick_sort(vector<int> &nums, int left, int right) {
        if (left >= right) {
            return;
        }

        int pivot = partition(nums, left, right);

        quick_sort(nums, left, pivot);
        quick_sort(nums, pivot + 1, right);
    }

    int partition(vector<int> &nums, int left, int right) {
        int pivot = nums[(left + right) / 2];

        int i = left - 1;
        int j = right + 1;

        while (i < j) {
            do {
                i += 1;
            } while (nums[i] < pivot);

            do {
                j -= 1;
            } while (nums[j] > pivot);

            if (i < j) {
                swap(nums[i], nums[j]);
            }
        }

        return j;
    }
};

/*
Solution 2:

Merge sort

Time: O(n log n) | Space: O(n)

- n: length of nums
*/

class Solution {
public:
    vector<int> sortArray(vector<int>& nums) {
        merge_sort(nums);
        return nums;
    }

    void merge_sort(vector<int> &nums) {
        if (nums.size() == 1) {
            return;
        }

        int n = nums.size();

        vector<int> left = vector<int> (nums.begin(), nums.begin() + n / 2);
        vector<int> right = vector<int> (nums.begin() + n / 2, nums.end());

        merge_sort(left);
        merge_sort(right);

        int left_idx = 0;
        int right_idx = 0;

        while (left_idx + right_idx != n) {
            if (left_idx == left.size()) {
                nums[left_idx + right_idx] = right[right_idx];
                right_idx += 1;
            } else if (right_idx == right.size()) {
                nums[left_idx + right_idx] = left[left_idx];
                left_idx += 1;
            } else if (left[left_idx] <= right[right_idx]) {
                nums[left_idx + right_idx] = left[left_idx];
                left_idx += 1;
            } else {
                nums[left_idx + right_idx] = right[right_idx];
                right_idx += 1;
            }
        }

        return;
    }
};