class Solution {
public:
    string countOfAtoms(string formula) {
        /*
        Time: O(n log n) | Space: O(n)

        where 'n' is the length of 'formula'
        */
        map<string, int> map; // time complexity is O(n log n) because 'map' is sorted
        vector<int> multipliers;
        bool prev_is_num = false;

        while (formula.size() > 0) {
            int last = formula.size() - 1;

             if (formula[last] == ')') {
                prev_is_num = false;
            } else if (formula[last] == '(') {
                if (!multipliers.empty()) {
                    multipliers.pop_back();
                }
                prev_is_num = false;
            } else if (formula[last] < 58) {
                while (formula[last - 1] < 58 && formula[last - 1] >= 48) {
                    last -= 1;
                }

                int to_push = atoi(formula.substr(last, formula.size() - last).c_str());

                if (!multipliers.empty()) {
                    to_push *= multipliers.back();
                }

                multipliers.push_back(to_push);
                prev_is_num = true;

            } else {
                while (!(formula[last] >= 65 && formula[last] <= 90)) {
                    last -= 1;
                }

                string element = formula.substr(last, formula.size() - last);

                if (prev_is_num) {
                    map[element] += multipliers.back();
                    multipliers.pop_back();
                } else if (multipliers.empty()) {
                    map[element] += 1;
                } else {
                    map[element] += multipliers.back();
                }
                
                
                prev_is_num = false;
            }

            while (formula.size() > last) {
                formula.pop_back();
            }
            
        }

        string res;

        for (const auto& [k, v]: map) {
            res += k;
            if (v != 1) {
                res += to_string(v);
            }
        }

        return res;
    }
};