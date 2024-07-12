class Solution {
public:
    int maximumGain(string s, int x, int y) {
        /*
        Time: O(n) | Space: O(n)

        where 'n' is the length of 's'
        */
        vector<char> chars;

        int res = 0;

        for (int i = 0; i < s.size(); ++i) {
            chars.push_back(s[i]);
        }

        if (x > y) {
            chars = helper(chars, true, x, &res);
            chars = helper(chars, false, y, &res);
        } else {
            chars = helper(chars, false, y, &res);
            chars = helper(chars, true, x, &res);
        }

        return res;


    }

    vector<char> helper(vector<char> chars, bool x, int step, int* sum) {
        vector<char> ret;

        if (x) {
            for (int i = 0; i < chars.size(); ++i) {
                if (!ret.empty() && ret.back() == 'a' && chars[i] == 'b') {
                    ret.pop_back();
                    *sum += step;
                } else {
                    ret.push_back(chars[i]);
                }
            }
        } else {
            for (int i = 0; i < chars.size(); ++i) {
                if (!ret.empty() && ret.back() == 'b' && chars[i] == 'a') {
                    ret.pop_back();
                    *sum += step;
                } else {
                    ret.push_back(chars[i]);
                }
            }
        }

        return ret;
    }
};