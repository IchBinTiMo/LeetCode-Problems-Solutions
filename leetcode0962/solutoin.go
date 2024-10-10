/*
Solution: Monotonic Stack

Time: O(n) | Space: O(n)

Runtime: 48 ms | 31.58%
Memory: 7.22 MB | 47.37%

- n: length of 'nums'
*/

func maxWidthRamp(nums []int) int {
    n := len(nums);

    stack := []int {};

    res := 0;

    for i := 0; i < n; i++ {
        if len(stack) == 0 || nums[stack[len(stack) - 1]] > nums[i] {
            stack = append(stack, i);
        }
    }

    for i:= n - 1; i >= 0; i-- {
        for len(stack) > 0 {
            left := stack[len(stack) - 1];
            if nums[left] <= nums[i] {
                if res < i - left {
                    res = i - left;
                }
                stack = stack[:len(stack) - 1];
            } else {
                break;
            }
        }
    }

    return res;
}