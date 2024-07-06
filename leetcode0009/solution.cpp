class Solution {
public:
    bool isPalindrome(int x) {
        /*
        Time: O(n) | Space: O(n)

        where 'n' is the number of digits in 'x'
        */
        if (x < 0) {
            return false;
        }

        int len = 0;
        int tmp = x;

        while (tmp > 0) {
            tmp /= 10;
            len += 1;
        }

        if (len == 1 || len == 0) {
            return true;
        }

        vector<int> vec;

        while (vec.size() < ceil(len / 2.0)) {
            int rem = x % 10;

            vec.push_back(rem);
            x /= 10;
        }

        if (len & 1) {
            vec.pop_back();
        }

        while (x > 0 && !vec.empty()) {
            int rem = x % 10;
            x /= 10;

            if (rem == vec.back()) {
                vec.pop_back();
            } else {
                return false;
            }
        }

        return (vec.size() == 0);
    }
};