/*
Solution 1:

Time: O(n) | Space: O(n)

- n: length of 'nums'
*/

class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        int n = nums.size();

        k %= n;

        deque<int> dq(nums.begin(), nums.end());

        for (int i = 0; i < k; ++i) {
            int tail = dq.back();

            dq.pop_back();

            dq.push_front(tail);
        }

        for (int i = 0; i < n; ++i) {
            nums[i] = dq[i];
        }
    }
};

/*
Solution 2:

Time: O(n) | Space: O(n)

- n: length of 'nums'
*/

class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        int n = nums.size();

        k %= n;
        
        vector<int> tmp(n);

        for (int i = 0; i < n; ++i) {
            tmp[i] = nums[(n + i - k) % n];
        }

        nums = tmp;
    }
};

/*
Solution 3:

Time: O(n) | Space: O(1)

- n: length of 'nums'
*/

class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        int n = nums.size();

        k %= n;

        for (int i = 0; i < n / 2; ++i) {
            swap(nums[i], nums[n - i - 1]);
        }

        for (int i = 0; i < k / 2; ++i) {
            swap(nums[i], nums[k - i - 1]);
        }

        for (int i = k; i < (n + k) / 2; ++i) {
            swap(nums[i], nums[n + k - i - 1]);
        }
    }
};