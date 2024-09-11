/*
Solution:

Time: O(1) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 7.62 MB | 44.87%

- n: length of 'start' and 'goal'
*/
class Solution {
public:
    int minBitFlips(int start, int goal) {
        int tmp = start ^ goal;
        int res = 0;

        while (tmp) {
            res += (tmp & 1);
            tmp >>= 1;
        }

        return res;
    }
};