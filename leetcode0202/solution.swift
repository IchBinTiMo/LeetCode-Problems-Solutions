class Solution {
    func isHappy(_ n: Int) -> Bool {
        var n = n;
        var visited: Set<Int> = [];

        while (n != 1) {
            var sum = 0;

            while (n != 0) {
                var mod = n % 10;

                sum += mod * mod;
                n /= 10;
            }

            if (visited.contains(sum)) {
                return false;
            } else {
                visited.insert(sum);
                n = sum;
            }
        }

        return true;
    }
}