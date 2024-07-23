/*
Solution 1:

Time: O(n log n) | Space: O(n)

- n: length of nums
- sort: O(n log n)
*/

class Solution {
public:
    vector<int> frequencySort(vector<int>& nums) {
        map<int, int> freqs;

        for (auto &num: nums) {
            freqs[num] += 1;
        }

        sort(nums.begin(), nums.end(), [&](int a, int b) {
            if (freqs[a] != freqs[b]) {
                return freqs[a] < freqs[b];
            } else {
                return a > b;
            }
        });

        return nums;

        
    }
};