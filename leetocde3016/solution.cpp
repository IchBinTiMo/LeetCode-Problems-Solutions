/*
Solution 1:

Time: O(n * 26) | Space: O(26)

- n: length of 'word'
*/

class Solution {
public:
    int minimumPushes(string word) {
        vector<int> freqs (26, 0);

        for (char &c: word) {
            freqs[c - 'a'] += 1;
        }

        sort(freqs.begin(), freqs.end());

        int res = 0;

        int mapped = 0; // the count of letter we have mapped

        while (!freqs.empty() && freqs.back() > 0) {
            int freq = freqs.back();
            freqs.pop_back();

            res += ((mapped / 8) + 1) * freq;
            mapped += 1;
        }

        return res;
    }
};