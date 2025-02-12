/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 12.92 MB | 5.17%

- n: length of 'nums'
*/

func maximumSum(nums []int) int {
	var maxs []int

	res := -1

	for i := 0; i < 100; i++ {
		maxs = append(maxs, 0)
	}

	for _, num := range nums {
		n := num
		sum := 0

		for n > 0 {
			sum += n % 10
			n /= 10
		}

		if maxs[sum] > 0 {
			res = max(res, maxs[sum]+num)
		}

		maxs[sum] = max(maxs[sum], num)
	}

	return res
}