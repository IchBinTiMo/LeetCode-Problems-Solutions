/*
Solution 1:

Time: O(1) | Space: O(1)

Runtime: 3 ms | 41.07%
Memory: 7.34 MB | 65.77%
*/

class Solution {
public:
    int findComplement(int num) {
        int digit = 0;
        int tmp = num;

        while (tmp > 0) {
            tmp >>= 1;
            digit += 1;
        }

        tmp = INT_MAX;
        tmp >>= 31 - digit;

        return num ^ tmp;
    }
};