class Solution:
    def longestPalindrome(self, s: str) -> str:
        if s == s[::-1]: return s
        
        start, size = 0, 1
        for i in range(1, len(s)):
            l, r = i - size, i + 1
            s1, s2 = s[l-1:r], s[l:r]
            print(s1, s2, f"\t{s[start: start + size]}")
            
            if l - 1 >= 0 and s1 == s1[::-1]:
                start, size = l - 1, size + 2
            elif s2 == s2[::-1]:
                start, size = l, size + 1
                
        return s[start:start+size]
# class Solution:
#     def longestPalindrome(self, s: str) -> str:
#         ans = ""
#         n = len(s)

#         if n == 1:
#             return s

#         def expand(left, right):
#             while left >= 0 and right < len(s) and s[left] == s[right]:
#                 left -= 1
#                 right += 1
#             return s[left + 1 : right]

#         for i in range(n):
#             tmp = expand(i, i)
#             if len(tmp) > len(ans):
#                 ans = tmp
#             tmp = expand(i, i + 1)
#             if len(tmp) > len(ans):
#                 ans = tmp

#         return ans