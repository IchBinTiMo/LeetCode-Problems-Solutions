/*
Solution:

Time: O(m * n) | Space: O(m * n)

Runtime: 85 ms | 79.17%
Memory: 81.34 MB | 62.50%

- m: length of 'mat'
- n: length of 'mat[0]'
*/

/**
 * @param {number[]} arr
 * @param {number[][]} mat
 * @return {number}
 */
var firstCompleteIndex = function(arr, mat) {
    let m = mat.length;
    let n = mat[0].length;

    let pos = Array(m * n + 1).fill(0);

    let row_visited = Array(m).fill(0);
    let col_visited = Array(n).fill(0);

    for (let i = 0; i < m; ++i) {
        for (let j = 0; j < n; ++j) {
            pos[mat[i][j]] = [i, j];
        }
    }

    for (let i = 0; i < arr.length; ++i) {
        let num = arr[i];

        let [r, c] = pos[num];

        row_visited[r] += 1;
        col_visited[c] += 1;

        if (row_visited[r] == n || col_visited[c] == m) {
            return i;
        }
    }
};