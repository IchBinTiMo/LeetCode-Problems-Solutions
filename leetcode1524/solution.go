/*
Solution: DP, Prefix Sum

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 10.15 MB | 64.71%

- n: length of 'arr'
*/

func numOfSubarrays(arr []int) int {
	odd := 0
	even := 0

	prev_odd := 0
	prev_even := 0

	for _, num := range arr {
		if num&1 == 1 {
			prev_odd, prev_even = prev_even+1, prev_odd
		} else {
			prev_even += 1
		}

		odd = (odd + prev_odd) % 1_000_000_007
		even = (even + prev_even) % 1_000_000_007
	}

	return odd
}