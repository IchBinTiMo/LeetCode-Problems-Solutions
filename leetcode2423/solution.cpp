/*
Solution 1:

Time: O(n) | Space: O(n)

- n: length of word
*/

class Solution {
public:
    bool equalFrequency(string word) {
        map<char, int> c_freqs;

        for (auto &c: word) {
            c_freqs[c] += 1;
        }

        map<int, int> cnt_freqs;

        for (auto it = c_freqs.begin(); it != c_freqs.end(); ++it) {
            cnt_freqs[it->second] += 1;
        }

        if (cnt_freqs.size() > 2) {
            return false;
        }

        int left = cnt_freqs.begin()->first;
        int right = cnt_freqs.rbegin()->first;

        if (right - left >= 1) {
            if ((left == 1 && cnt_freqs[left] == 1) || (right - left == 1 && cnt_freqs[right] == 1)) {
                return true;
            }
        } else if (right == left) {
            if (right == 1) {
                return true;
            } else {
                if (cnt_freqs[right] == 1) {
                    return true;
                }
            }
        }

        return false;
    }
};