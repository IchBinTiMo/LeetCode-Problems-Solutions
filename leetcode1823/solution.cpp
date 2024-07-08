class Solution {
public:
    int findTheWinner(int n, int k) {
        /*
        Brute force solution
        Time O(n * k) | Space O(n)
        */
        vector<bool> visited(n, false);

        int current = 0;
        int lose = 0;

        while (n - lose > 1) {
            int step = 0;

            while (step < k) {
                if (!visited[(current + step) % n]) {
                    step += 1;
                } else {
                    current += 1;
                }
            }

            visited[(current + step - 1) % n] = true;
            current = (current + step) % n;
            lose += 1;
            
        }

        for (int i = 0; i < n; ++i) {
            if (!visited[i]) {
                return i + 1;
            }
        }

        return 1;
    }
};