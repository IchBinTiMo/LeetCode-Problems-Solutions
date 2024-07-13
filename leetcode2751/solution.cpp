class Solution {
public:
    vector<int> survivedRobotsHealths(vector<int>& positions, vector<int>& healths, string directions) {
        /*
        Time: O(n log n) | Space: O(n)

        where 'n' is the length of 'positions'
        */
        vector<pair<int, int>> robots;

        for (int i = 0; i < positions.size(); ++i) {
            robots.push_back({positions[i], i});
        }

        sort(robots.begin(), robots.end());

        vector<pair<int, int>> stack;

        for (int i = 0; i < robots.size(); ++i) {
            if (stack.empty()) {
                stack.push_back(robots[i]);
            } else {
                int prev = stack.back().second;
                int current = robots[i].second;

                if (!(directions[prev] == 'R' && directions[current] == 'L')) {
                    stack.push_back(robots[i]);
                    continue;
                }

                while (directions[prev] == 'R' && directions[current] == 'L') {
                    if (healths[prev] > healths[current]) {
                        healths[prev] -= 1;
                        break;
                    } else if (healths[prev] == healths[current]) {
                        stack.pop_back();
                        break;
                    } else {
                        healths[current] -= 1;
                        stack.pop_back();
                        if (stack.empty()) {
                            stack.push_back(robots[i]);
                            break;
                        }
                        prev = stack.back().second;

                        if (!(directions[prev] == 'R' && directions[current] == 'L')) {
                            stack.push_back(robots[i]);
                            break;
                        }
                    }
                }
            }
        }

        sort(stack.begin(), stack.end(), [](auto& left, auto& right) {
            return left.second < right.second;
        });

        vector<int>  res;

        for (int i = 0; i < stack.size(); ++i) {
            res.push_back(healths[stack[i].second]);
        }

        return res;
    }
};