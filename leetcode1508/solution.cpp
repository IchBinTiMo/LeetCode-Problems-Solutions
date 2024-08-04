/*
Solution 1: Brute Force

Time: O(n ^ 2) | Space: O(1)

- n: length of nums
*/

class Solution {
public:
    int rangeSum(vector<int>& nums, int n, int left, int right) {
        vector<int> sums;

        for (int i = 0; i < n; ++i) {
            int s = 0;
            for (int j = i; j < n; ++j) {
                s += nums[j];
                sums.push_back(s);
            }
        }

        sort(sums.begin(), sums.end());

        int res = 0;

        for (int i = left; i <= right; ++i) {
            res += sums[i - 1];
            res %= 1000000007;
        }

        return res;
    }
};

/*
Solution 2: Segment Tree

Time: O((n ^ 2) log n) | Space: O(n)

- n: length of nums
*/

class Solution {
    vector<long long> tree;
    int len;
public:
    int rangeSum(vector<int>& nums, int n, int left, int right) {
        len = nums.size();
        tree.resize(4 * len, 0);

        build(nums, 1, 0, len);

        vector<long long> sums;

        for (int i = 0; i < len; ++i) {
            for (int j = i + 1; j <= len; ++j) {
                sums.push_back(query(i, j));
            }
        }

        sort(sums.begin(), sums.end());

        int res = 0;

        for (int i = left; i <= right; ++i) {
            res += sums[i - 1];
            res %= 1000000007;
        }

        return res;
    }

    int left(int n) {
        return 2 * n;
    }

    int right(int n) {
        return 2 * n + 1;
    }

    void build(vector<int>& nums, int v, int l, int r) {
        if (r - l == 1) {
            tree[v] = nums[l];
        } else {
            int m = (l + r) / 2;

            build(nums, left(v), l, m);
            build(nums, right(v), m, r);

            tree[v] = tree[left(v)] + tree[right(v)];
        }
    }

    int query(int ql, int qr, int v, int l, int r) {
        if (ql == l && qr == r) {
            return tree[v];
        }

        int mid = (l + r) / 2;

        if (qr <= mid) {
            return query(ql, qr, left(v), l, mid);
        } else if (ql >= mid) {
            return query(ql, qr, right(v), mid, r);
        } else {
            return query(ql, mid, left(v), l, mid) + query(mid, qr, right(v), mid, r);
        }
    }

    int query(int l, int r) {
        return query(l, r, 1, 0, len);
    }
};

