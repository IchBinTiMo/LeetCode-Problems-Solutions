"""
Solution 1: 

TIme: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 17.93MB | 9.90%

- n: length of 'A'
""" 

class Solution:
    def findThePrefixCommonArray(self, A: List[int], B: List[int]) -> List[int]:
        n = len(A)
        res = []

        current = 0

        a = set()
        b = set()

        for i in range(0, n):
            if A[i] == B[i]:
                current += 1
            else:
                current += (A[i] in b) + (B[i] in a)

            a.add(A[i])
            b.add(B[i])

            res.append(current)

        return res
    
"""
Solution 2:

Time: O(n ^ 2) | Space: O(n)

Runtime: 35 ms | 28.39%
Memory: 17.38MB | 62.76%

- n: length of 'A'
"""

class Solution:
    def findThePrefixCommonArray(self, A: List[int], B: List[int]) -> List[int]:
        n = len(A)
        res = []

        a = []
        b = []

        for i in range(0, n):
            a.append(A[i])
            b.append(B[i])

            res.append(len(set(a) & set(b)))

        return res