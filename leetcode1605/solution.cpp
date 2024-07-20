/*
Solution 1:

Time: O(m * n) | Space: O(m * n)

- m: Length of rowSum
- n: Length of colSum
*/
class Solution {
public:
    vector<vector<int>> restoreMatrix(vector<int>& rowSum, vector<int>& colSum) {
        int m = rowSum.size();
        int n = colSum.size();
        vector<vector<int>> res(m, vector<int> (n, 0));

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                res[i][j] += min(rowSum[i], colSum[j]);
                rowSum[i] -= res[i][j];
                colSum[j] -= res[i][j];
            }
        }

        return res;        
    }
};


/*
Solution 2: 

Time: O(m + n log n) | Space: O(m * n)

- m: Length of rowSum
- n: Length of colSum
*/
class Solution {
public:
    vector<vector<int>> restoreMatrix(vector<int>& rowSum, vector<int>& colSum) {
        int m = rowSum.size();
        int n = colSum.size();
        vector<vector<int>> res(m, vector<int> (n, 0));

        vector<int> col_idx;

        for (int j = 0; j < n; ++j) {
            col_idx.push_back(j);
        }

        // sort col_idx in descending order based on colSum[col_idx]
        sort(col_idx.begin(), col_idx.end(), [&](int a, int b) {
            return colSum[a] > colSum[b];
        });

        for (int i = 0; i < m; ++i) {
            while (rowSum[i] > 0) {
                int val = min(rowSum[i], colSum[col_idx.back()]);

                res[i][col_idx.back()] += val;
                rowSum[i] -= val;
                colSum[col_idx.back()] -= val;
                
                if (colSum[col_idx.back()] == 0) {
                    col_idx.pop_back();
                }
            }
        }

        return res;        
    }
};