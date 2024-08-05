/*
Solution 1:

Time: O(n) | Space: O(n)

- n: length of arr
*/

class Solution {
public:
    string kthDistinct(vector<string>& arr, int k) {
        map<string, int> freqs;

        for (auto &s: arr) {
            freqs[s] += 1;
        }

        for (auto &s: arr) {
            if (freqs[s] == 1) {
                if (k == 1) {
                    return s;
                }
                k -= 1;
            }
        }

        return "";
    }
};