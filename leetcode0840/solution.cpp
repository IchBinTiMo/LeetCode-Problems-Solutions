/*
Solution 1: brute force

Time: O(m * n) | Space: O(1)

- m: Length of grid
- n: Length of grid[0]

*/

class Solution {
public:
    int numMagicSquaresInside(vector<vector<int>>& grid) {
        int m = grid.size();
        int n = grid[0].size();

        if (m < 3 || n < 3) {
            return 0;
        }

        int res = 0;

        for (int i = 1; i < m - 1; ++i) {
            for (int j = 1; j < n - 1; ++j) {
                set<int> nums;
                bool valid = true;

                for (int di = -1; di <= 1; ++di) {
                    for (int dj = -1; dj <= 1; ++dj) {
                        if (grid[i + di][j + dj] < 1 || grid[i + di][j + dj] > 9) {
                            valid = false;
                        }

                        nums.insert(grid[i + di][j + dj]);
                    }
                }

                if (nums.size() != 9 || !valid) {
                    continue;
                }

                if (grid[i - 1][j - 1] + grid[i - 1][j] + grid[i - 1][j + 1] != 15) {
                    continue;
                }

                if (grid[i][j - 1] + grid[i][j] + grid[i][j + 1] != 15) {
                    continue;
                } 

                if (grid[i + 1][j - 1] + grid[i + 1][j] + grid[i + 1][j + 1] != 15) {
                    continue;
                }

                if (grid[i - 1][j - 1] + grid[i][j - 1] + grid[i + 1][j - 1] != 15) {
                    continue;
                }

                if (grid[i - 1][j] + grid[i][j] + grid[i + 1][j] != 15) {
                    continue;
                }

                if (grid[i - 1][j + 1] + grid[i][j + 1] + grid[i + 1][j + 1] != 15) {
                    continue;
                }

                if (grid[i - 1][j - 1] + grid[i][j] + grid[i + 1][j + 1] != 15) {
                    continue;
                }

                if (grid[i + 1][j - 1] + grid[i][j] + grid[i - 1][j + 1] != 15) {
                    continue;
                }

                res += 1;

            }
        }

        return res;
    }
};