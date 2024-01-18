class Solution:
    def climbStairs(self, n: int) -> int:
        prev = 1
        current = 1

        for i in range(2, n + 1, 1):
            prev, current = current, prev + current

        return current
