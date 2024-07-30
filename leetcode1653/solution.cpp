/*
Solution 1:

Time: O(n) | Space: O(1)

- n: length of 's'
*/

class Solution {
public:
    int minimumDeletions(string s) {
        int n = s.size();

        int cnt_a = 0;
        int cnt_b = 0;

        bool imbalance = false; // true if at least one 'b' is followed by 1 or more 'a'

        int res = 0;

        for (int i = 0; i < n; ++i) {
            
            if (s[i] == 'a') {
                if (imbalance) {
                    cnt_a += 1;
                }
            } else {
                if (imbalance && cnt_a > cnt_b) {
                    res += cnt_b;
                    cnt_a = 0;
                    cnt_b = 0;
                }

                cnt_b += 1;
                imbalance = true;
            }
        }


        // if 's' ends with an 'a'
        if (imbalance) {
            res += min(cnt_a, cnt_b);
        }

        return res;
    }
};

