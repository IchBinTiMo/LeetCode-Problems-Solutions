/*
Solution 1:

DP

Time: O(n ^ 2) | Space: O(n)

- n: length of 'books'
*/

class Solution {
public:
    int minHeightShelves(vector<vector<int>>& books, int shelfWidth) {
        int n = books.size();

        int dp[n + 1];

        dp[0] = 0;

        for (int i = 1; i <= n; ++i) {
            int w = books[i - 1][0];
            int h = books[i - 1][1];

            // we initially assume that we put the current book into a new shelf
            dp[i] = dp[i - 1] + h;

            // iterate to grab previous books into current shelf until w > shelfWidth
            for (int j = i - 1; j > 0; --j) {
                w += books[j - 1][0];

                if (w > shelfWidth) {
                    break;
                }

                // update the height of the current shelf
                h = max(h, books[j - 1][1]);

                // update the minimum height until current shelf
                dp[i] = min(dp[i], dp[j - 1] + h);
            }
        }
        
        return dp[n];
    }
};