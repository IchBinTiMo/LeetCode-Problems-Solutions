/*
Solution:

Time: O(n^2) | Space: O(n^2)

Runtime: 139 ms | 100.00%
Memory: 41.49 MB | 100.00%

- n: length of 'nums'
*/

func tupleSameProduct(nums []int) int {
	n := len(nums)
	m := make(map[int]int)

	res := 0

	for i := range n {
		for j := i + 1; j < n; j++ {
			prd := nums[i] * nums[j]

			m[prd] += 1
			res += (m[prd] - 1) * 8
		}
	}

	return res
}