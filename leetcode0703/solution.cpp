/*
Solution 1: Brute Force

Time: O(m * n) | Space: O(n)

- m: # of calls
- n: length of nums
*/

class KthLargest {
public:
    int kth;
    int count;
    map<int, int> freq;

    KthLargest(int k, vector<int>& nums) {
        count = nums.size();
        kth = k;
        for (auto &num: nums) {
            freq[num] += 1;
        }
    }
    
    int add(int val) {
        freq[val] += 1;
        count += 1;

        int place = 0;

        for (auto it = freq.rbegin(); it != freq.rend(); ++it) {
            if (place + it->second >= kth) {
                return it->first;
            } else {
                place += it->second;
            }
        }

        return -1;
    }
};

/**
 * Your KthLargest object will be instantiated and called as such:
 * KthLargest* obj = new KthLargest(k, nums);
 * int param_1 = obj->add(val);
 */