class Solution {
public:
    int findTheWinner(int n, int k) {
        /* Recursion % DP
        Time O(n) | Space O(n)
        */
        return recursion(n, k) + 1;
    }

    int recursion(int n, int k) {
        /* Space Complexity = O(n) is due to the call stack */
        if (n == 1) {
            return 0;
        } else {
            return (recursion(n - 1, k) + k) % n;
        }
    }
};

// class Solution {
// public:
//     int findTheWinner(int n, int k) {
//         /*
//         Brute force solution
//         Time O(n * n) | Space O(n)
//         */
//         vector<int> vec;
//         int current = 0;

//         for (int i = 0; i < n; ++i) {
//             vec.push_back(i + 1);
//         }

//         while (vec.size() > 1) {
//             current = (current + k - 1) % n;
//             vec.erase(vec.begin() + current); // this is why the time complexity is O(n * n)
//             n -= 1;
//         }

//         return vec[0];
//     }
// };

// // class Solution {
// // public:
// //     int findTheWinner(int n, int k) {
// //         /*
// //         Brute force solution
// //         Time O(n * k) | Space O(n)
// //         */
// //         vector<bool> visited(n, false);

// //         int current = 0;
// //         int lose = 0;

// //         while (n - lose > 1) {
// //             int step = 0;

// //             while (step < k) {
// //                 if (!visited[(current + step) % n]) {
// //                     step += 1;
// //                 } else {
// //                     current += 1;
// //                 }
// //             }

// //             visited[(current + step - 1) % n] = true;
// //             current = (current + step) % n;
// //             lose += 1;
            
// //         }

// //         for (int i = 0; i < n; ++i) {
// //             if (!visited[i]) {
// //                 return i + 1;
// //             }
// //         }

// //         return 1;
// //     }
// // };