class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        c = Counter(nums)
        l = len(nums) // 2

        for k, v in c.items():
            if v > l:
                return k

# class Solution:
#     def majorityElement(self, nums: List[int]) -> int:
#         if len(nums) == 1:
#             return nums[0] 
#         nums.sort()
#         count = 1
#         for i in range(len(nums) - 1):
#             if nums[i] == nums[i + 1]:
#                 count += 1
#                 if count > len(nums) // 2:
#                     return nums[i]
#             else:
#                 count = 1
            

        