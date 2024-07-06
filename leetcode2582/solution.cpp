class Solution {
public:
    int passThePillow(int n, int time) {
        /*
        Time: O(1) | Space: O(1)
        */
        int dir = time / (n - 1);
        int rem = time % (n - 1);

        if (dir & 1 == 1) {
            return n - rem;
        } else {
            return 1 + rem;
        }
    }
};