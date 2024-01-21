class Solution:
    def rob(self, nums: List[int]) -> int:
        prev_prev= 0
        prev = nums[0]

        for i in range(1, len(nums)):
            prev_prev, prev = prev, max(prev, prev_prev + nums[i])

        return prev

# class Solution:
#     def rob(self, nums: List[int]) -> int:
#         if(len(nums) <= 2):
#             return max(nums)
#         if len(nums) % 2:
#             nums.append(0)
#         for i in range(1, int(len(nums) / 2)):
#             if i >= 2:
#                 nums[i * 2] += max(nums[(i - 1) * 2], nums[(i - 2) * 2 + 1])
#             else:
#                 nums[i * 2] += nums[(i - 1) * 2]
#             nums[i * 2 + 1] += max(nums[(i - 1) * 2] , nums[(i - 1) * 2 + 1])

          
#         return max(nums[-1], nums[-2])