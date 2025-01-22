/*
Solution: BFS

Time: O(m * n) | Space: O(m * n)

Runtime: 23 ms | 94.44%
Memory: 44.12 MB | 83.33%

- m: rows of 'isWater'
- n: columns of 'isWater[0]'
*/

import (
	"math"
)

func highestPeak(isWater [][]int) [][]int {
	m := len(isWater)
	n := len(isWater[0])

	dir := []int{0, 1, 0, -1, 0}

	q := []int{}

	for i := range isWater {
		for j := range isWater[i] {
			if isWater[i][j] == 1 {
				isWater[i][j] = 0
				q = append(q, i*n+j)
			} else {
				isWater[i][j] = math.MaxInt32
			}
		}
	}

	height := 0

	for ; len(q) > 0; height++ {
		prev := q

		for _, front := range prev {
			i, j := front/n, front%n

			for k := range 4 {
				new_i := i + dir[k]
				new_j := j + dir[k+1]

				if new_i >= 0 && new_i < m &&
					new_j >= 0 && new_j < n &&
					height+1 < isWater[new_i][new_j] {
					isWater[new_i][new_j] = height + 1
					q = append(q, new_i*n+new_j)
				}
			}
		}

		q = q[len(prev):]
	}

	return isWater
}