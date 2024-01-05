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