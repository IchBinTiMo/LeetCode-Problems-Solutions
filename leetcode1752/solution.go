/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.22 MB | 77.27%

- n: length of nums
*/

func check(nums []int) bool {
	n := len(nums)
	cnt := 0

	for i := range n {
		if nums[i] > nums[(i+1)%n] {
			cnt += 1
		}
	}

	return cnt < 2
}