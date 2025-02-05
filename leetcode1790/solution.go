/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 3.90 MB | 100.00%

- n: length of 's1'
*/

func areAlmostEqual(s1 string, s2 string) bool {
	cnt := 0
	freq := 0

	for i := range len(s1) {
		if s1[i] != s2[i] {
			cnt += 1
		}

		freq += 1 << (s1[i] - 'a')
		freq -= 1 << (s2[i] - 'a')
	}

	return (cnt == 0 || cnt == 2) && freq == 0
}