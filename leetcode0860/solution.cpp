/*
Solution 1:

Time: O(n) | Space: O(1)

Runtime: 53 ms, 97.85%
Memory Usage: 86.13 MB, 9.36%

- n: length of bills
*/

class Solution {
public:
    bool lemonadeChange(vector<int>& bills) {
        if (bills[0] != 5) {
            return false;
        }

        int change[2] = {0, 0};

        for (int i = 0; i < bills.size(); ++i) {
            if (bills[i] == 5) {
                change[0] += 5;
            } else if (bills[i] == 10) {
                if (change[0] < 5) {
                    return false;
                }
                change[0] -= 5;
                change[1] += 10;
            } else {
                if (change[1] >= 10) {
                    if (change[0] >= 5) {
                        change[0] -= 5;
                        change[1] -= 10;
                    } else {
                        return false;
                    }
                } else if (change[0] >= 15) {
                    change[0] -= 15;
                } else {
                    return false;
                }
            }
        }
        return true;
    }
};