class Solution {
public:
    int findCenter(vector<vector<int>>& edges) {
        /// Time: O(1) | Space: O(1)
        if (edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] ) {
            return edges[0][0];
        } else {
            return edges[0][1];
        }
    }
};