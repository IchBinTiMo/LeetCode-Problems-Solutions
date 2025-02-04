/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 4.15 MB | 11.54%

- n: length of nums
*/

func maxAscendingSum(nums []int) int {
	current := 0
	res := 0

	var q []int

	for _, num := range nums {
		if len(q) > 0 && q[len(q)-1] >= num {
			q = q[:0]
			current = 0
		}

		q = append(q, num)
		current += num

		if res < current {
			res = current
		}
	}

	return res
}