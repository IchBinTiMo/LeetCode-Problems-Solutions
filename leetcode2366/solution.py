class Solution:
    def minimumReplacement(self, nums: List[int]) -> int:
        last = nums[-1]
        n = len(nums)
        step = 0

        for i in range(n-1, 0, -1):
            if last >= nums[i - 1]:
                last = nums[i - 1]
                continue
            
            tmp = nums[i - 1] // last

            if nums[i - 1] % last:
                tmp += 1

            last = nums[i - 1] // tmp
            step += tmp-1

        return step
