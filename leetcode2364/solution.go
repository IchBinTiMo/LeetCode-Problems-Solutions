/*
Solution: Hashmap

Time: O(n) | Space: O(n)

Runtime: 32 ms | 61.54%
Memory: 14.32 MB | 61.54%

- n: length of 'nums'
*/

func countBadPairs(nums []int) int64 {
	n := len(nums)

	ht := make(map[int]int)

	cnt := 0

	for i, val := range nums {
		diff := i - val

		cnt += ht[diff]

		ht[diff] += 1
	}

	return int64(n*(n-1)/2 - cnt)
}