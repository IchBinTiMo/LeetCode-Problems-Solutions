/*
Solution:

Time: O(m * n) | Space: O(1)

Runtime: 31 ms | 97.78%
Memory: 33.78 MB | 95.30%

- m: length of 'words'
- n: length of elements in 'words'
*/

class Solution {
public:
    int countConsistentStrings(string allowed, vector<string>& words) {
        int res = 0;

        int mask = 0;

        for (auto &c: allowed) {
            mask |= 1 << ((int)c - (int)('a'));
        }

        for (auto &word: words) {
            bool valid = true;

            for (auto &c: word) {
                if ((mask | 1 << ((int)c - (int)('a'))) != mask) {
                    valid = false;
                    break;
                }
            }

            if (valid) {
                res += 1;
            }
        }

        return res;
    }
};