class Solution:
    def minimumReplacement(self, nums: List[int]) -> int:
        ans = 0

        current = nums[-1]

        for i in range(len(nums) - 1, -1, -1):
            num = nums[i]

            if current >= num:
                current = num
                continue

            step = int(num / current) if num % current != 0 else int(num / current) - 1

            current = int(num / (step + 1))

            ans = ans + step

        return int(ans)

# class Solution:
#     def minimumReplacement(self, nums: List[int]) -> int:
#         last = nums[-1]
#         n = len(nums)
#         step = 0

#         for i in range(n-1, 0, -1):
#             if last >= nums[i - 1]:
#                 last = nums[i - 1]
#                 continue
            
#             tmp = nums[i - 1] // last

#             if nums[i - 1] % last:
#                 tmp += 1

#             last = nums[i - 1] // tmp
#             step += tmp-1

#         return step
