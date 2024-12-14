/*
Solution: Sliding Window + DP

TIme: O(n log n) | Space: O(n)

Runtime: 95 ms | 77.84%
Memory: 119.49 MB | 68.63%

- n: length of nums
*/

class Solution {
public:
    long long continuousSubarrays(vector<int>& nums) {
        int n = nums.size();

        map<int, int> cnt;

        vector<long long> dp (n + 1, 0);

        int left = 0;

        for (int right = 0; right < n; ++right) {
            int num = nums[right];
            
            if (cnt.find(num) == cnt.end()) {
                cnt.emplace(num, 1);
            } else {
                cnt[num] += 1;
            }

            int min = cnt.begin()->first;
            int max = (--cnt.end())->first;

            if (max - min > 2) {
                while (left < right && max - min > 2) {
                    int l = nums[left];
                    cnt[l] -= 1;


                    if (cnt[l] == 0) {
                        cnt.erase(l);
                    }

                    min = cnt.begin()->first;
                    max = (--cnt.end())->first;
                    left += 1;
                } 
            }

            dp[right + 1] = right - left + 1 + dp[right];
        }

        return dp[n];
    }
};