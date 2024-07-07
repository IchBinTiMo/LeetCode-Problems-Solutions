class Solution {
public:
    int numWaterBottles(int numBottles, int numExchange) {
        /*
        Time: O( log n ) | Space: O( 1 )
        
        n: numBottles
        */
        int res = 0;
        int rem = numBottles % numExchange;

        while (numBottles / numExchange > 0) {
            res += numBottles - rem;
            numBottles /= numExchange;
            numBottles += rem;
            rem = numBottles % numExchange;
        }

        res += rem;

        return res;
    }
};