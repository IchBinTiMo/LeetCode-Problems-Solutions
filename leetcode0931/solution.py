class Solution:
    def minFallingPathSum(self, matrix: List[List[int]]) -> int:
        for row in range(1, len(matrix)):
            for col in range(len(matrix[0])):
                matrix[row][col] += min(matrix[row - 1][max(0, col - 1):col + 2])

        return min(matrix[-1])