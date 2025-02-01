/*
Solutioni:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 4.66 MB | 82.93%

- n: length of nums
*/

func isArraySpecial(nums []int) bool {
	for i := 1; i < len(nums); i++ {
		if (nums[i-1]&1)^(nums[i]&1) == 0 {
			return false
		}
	}

	return true
}