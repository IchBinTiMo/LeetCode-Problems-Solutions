class Solution {
    public:
        int minOperations(vector<string>& logs) {
            /*
            Time: O(N) | Space: O(1)

            where 'N' is the length of 'logs'
             */
            int depth = 0;
    
            for (auto& log: logs) {
                if (log[0] == '.') {
                    if (log.size() == 3 && depth > 0) {
                        depth -= 1;
                    }
                } else {
                    depth += 1;
                }
            }
    
            return depth;
        }
    };