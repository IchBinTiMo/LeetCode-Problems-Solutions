/*
Solution: xor

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 10.37 MB | 66.67%

- n: length of 'derived'
*/

func doesValidArrayExist(derived []int) bool {
	res := 0

	for _, x := range derived {
		res ^= x
	}

	return res == 0
}