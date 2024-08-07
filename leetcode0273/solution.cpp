/*
Solution 1

Time: O(n) | Space: O(1)

- n: # of digits of 'num'
*/

class Solution {
public:
    string numberToWords(int num) {
        if (num == 0) {
            return "Zero";
        }

        map<int, string> nums = {
            {1, "One "},
            {2, "Two "},
            {3, "Three "},
            {4, "Four "},
            {5, "Five "},
            {6, "Six "},
            {7, "Seven "},
            {8, "Eight "},
            {9, "Nine "},
            {10, "Ten "},
            {11, "Eleven "},
            {12, "Twelve "},
            {13, "Thirteen "},
            {14, "Fourteen "},
            {15, "Fifteen "},
            {16, "Sixteen "},
            {17, "Seventeen "},
            {18, "Eighteen "},
            {19, "Nineteen "},
            {20, "Twenty "},
            {30, "Thirty "},
            {40, "Forty "},
            {50, "Fifty "},
            {60, "Sixty "},
            {70, "Seventy "},
            {80, "Eighty "},
            {90, "Ninety "},
            };

        string res = "";

        int cnt = 0;

        int sum = 0;

        while (num > 0) {
            string tmp = "";
            
            int d = num % 10;
            num /= 10;
            int t = num % 10;
            num /= 10;
            int h = num % 10;
            num /= 10;


            if (t == 1) {
                tmp = nums[t * 10 + d] + tmp;
            } else if (t == 0) {
                tmp = nums[d] + tmp;
            } else {
                tmp = nums[t * 10] + nums[d] + tmp;
            }

            if (h != 0) {
                tmp = nums[h] + "Hundred " + tmp;
            }


            sum = h + t + d;

            if (sum > 0) {
                if (cnt == 1) {
                    res = tmp + "Thousand " + res;
                } else if (cnt == 2) {
                    res = tmp + "Million " + res;
                } else if (cnt == 3) {
                    res = tmp + "Billion " + res;
                } else {
                    res = tmp + res;
                }
            } else {
                res = tmp + res;
            }

            cnt += 1;
            
        }

        res.pop_back();

        return res;
    }
};