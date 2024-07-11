class Solution {
public:
    string reverseParentheses(string s) {
        /*
        Recursion

        Time: O(n * k) | Space: O(n)

        where 'n' is the length of 's'
        and 'k' is the number of '('
        */
        string res = recursion(s, 0);
        int i = 0;
        while (i < res.size()) {
            if (res[i] == '(' || res[i] == ')') {
                res.erase(i, 1);
                continue;
            }

            i += 1;
        }
        return res;
    }

    string recursion(string s, int start) {
        string to_rev = "(";
        int i = start;
        while (i < s.size()) {
            if (s[i] == '(') {
                string reved = recursion(s, i + 1);
                to_rev += reved;
                i += reved.size();
            } else if (s[i] == ')') {
                to_rev += s[i];
                reverse(to_rev.begin(), to_rev.end());
                return to_rev;
            } else {
                to_rev += s[i];
                i += 1;
            }
        }

        return to_rev;
    }
};

