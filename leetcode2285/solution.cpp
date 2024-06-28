class Solution {
public:
    long long maximumImportance(int n, vector<vector<int>>& roads) {
        /// Time: O(N log N) | Space: O(N)
        /// where N is the number of roads
        vector<long long> freqs(n, 0);

        for(vector<int> &road: roads) {
            freqs[road[0]] += 1;
            freqs[road[1]] += 1;
        }

        long long res = 0;

        sort(freqs.begin(), freqs.end());

        for(long long i = 0; i < freqs.size(); ++i) {
            res += freqs[i] * (i + 1);
        }

        return res;
    }
};