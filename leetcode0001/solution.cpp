class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        /*
        Time: O(n) | Space: O(n)
        */
        unordered_map<int, int> map;

        for (int i = 0; i < nums.size(); ++i) {
            if (map.find(target - nums[i]) != NULL) {
                return vector<int> ({i, map[target - nums[i]]});
            } else {
                map.emplace(nums[i], i);
            }
        }

        return vector<int> ({-1, -1});
    }
};