/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 8.02 MB | 23.64%

- n: length of 'expression'
*/

class Solution {
public:
    string fractionAddition(string expression) {
        int n = expression.size();

        vector<int> aboves;
        vector<int> belows;

        int left = 0;
        int right = 0;

        while (right < n) {
            pair<int, int> num;
            while (expression[right] != '/') {
                right += 1;
            }

            string first = expression.substr(left, right - left);

            aboves.push_back(stoi(first));

            left = right + 1;
            right = left + 1;

            while (right < n && expression[right] != '+' && expression[right] != '-') {
                right += 1;
            }

            string second = expression.substr(left, right - left);
            belows.push_back(stoi(second));

            left = right;
            right = left + 1;
        }

        n = aboves.size();

        int below = 1;

        for (int i = 0; i < n; ++i) {
            if (gcd(belows[i], below) == 1) {
                below *= belows[i];
            } else {
                below = belows[i] * below / gcd(belows[i], below);
            }
        }

        int above = 0;

        for (int i = 0; i < n; ++i) {
            aboves[i] *= below / belows[i];
            above += aboves[i];
        }

        int gcf = gcd(below, above);

        above /= gcf;
        below /= gcf;

        return to_string(above) + "/" + to_string(below);
    }
};