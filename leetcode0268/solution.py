class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        s1 = sum(nums)
        s2 = int((len(nums)) * (len(nums) + 1) / 2)

        return s2 - s1
        