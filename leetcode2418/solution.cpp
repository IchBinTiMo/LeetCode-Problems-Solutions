/*
Solution 1:

Time: O(n log n) | Space: O(n)

- n: length of names
*/

class Solution {
public:
    vector<string> sortPeople(vector<string>& names, vector<int>& heights) {
        map<int, string> m;
        for (int i = 0; i < names.size(); ++i) {
            m.emplace(heights[i], names[i]);
        }

        vector<string> res;

        for (auto i = m.rbegin(); i != m.rend(); ++i) {
            res.push_back(i->second);
        }

        return res;
    }
};