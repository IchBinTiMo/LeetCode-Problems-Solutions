/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 49 ms | 88.89%
Memory: 25.00 MB | 66.67%

- n: length of 'queries'
*/

func queryResults(limit int, queries [][]int) []int {
	current := make(map[int]int)
	cnts := make(map[int]int)

	var res []int

	for _, q := range queries {
		ball := q[0]
		color := q[1]

		if prev, ok := current[ball]; ok {
			cnts[prev] -= 1

			if cnts[prev] == 0 {
				delete(cnts, prev)
			}
		}

		current[ball] = color

		cnts[color] += 1

		res = append(res, len(cnts))
	}

	return res
}