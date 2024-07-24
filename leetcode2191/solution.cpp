/*
Solution 1:

Time: O(n log n) | Space: O(n)

- n: length of nums
*/

class Solution {
public:
    vector<int> sortJumbled(vector<int>& mapping, vector<int>& nums) {
        map<int, int> mapped;
        map<int, int> indices;

        for (int i = 0; i < nums.size(); ++i) {
            if (mapped.find(nums[i]) != mapped.end()) {
                continue;
            } else {
                deque<int> dq;

                int num = nums[i];

                if (nums[i] == 0) {
                    num = mapping[0];
                } else {
                    while (num > 0) {
                        dq.push_back(num % 10);
                        num /= 10;
                    }

                    while (!dq.empty()) {
                        num *= 10;
                        num += mapping[dq.back()];
                        dq.pop_back();
                    }
                }

                mapped.emplace(nums[i], num);
                if (indices.find(nums[i]) != indices.end()) {
                    indices[nums[i]] = i;
                }
            }
        }

        if (mapped.size() == 1) {
            return nums;
        }

        sort(nums.begin(), nums.end(), [&](int a, int b) {
            if (a == b) {
                return a <= b;
            } else if (mapped[a] == mapped[b]) {
                return indices[a] < indices[b];
            } else {
                return mapped[a] < mapped[b];
            }
        });

        return nums;
    }
};