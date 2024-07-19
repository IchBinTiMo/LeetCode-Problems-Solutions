class Solution {
public:
    vector<int> luckyNumbers (vector<vector<int>>& matrix) {
        /*
        Time: O(m * n) | Space: O(m + n)

        where m is the number of rows and n is the number of columns in the matrix
        */
        vector<int> row(matrix.size(), INT_MAX);
        vector<int> col(matrix[0].size(), INT_MIN);

        vector<int> res;

        for (int i = 0; i < matrix.size(); ++i) {
            for (int j = 0; j < matrix[0].size(); ++j) {
                row[i] = min(row[i], matrix[i][j]);
                col[j] = max(col[j], matrix[i][j]);
            }
        }

        for (int i = 0; i < matrix.size(); ++i) {
            for (int j = 0; j < matrix[0].size(); ++j) {
                if (row[i] == col[j]) {
                    res.push_back(row[i]);
                }
            }
        }

        return res;
    }
};