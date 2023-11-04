import "dart:math";

class Solution {
  int getLastMoment(int n, List<int> left, List<int> right) {
    int l = left.isEmpty ? 0 : left.reduce(max);
    int r = right.isEmpty ? 0 : n - right.reduce(min);

    return l > r ? l : r;
  }
}
