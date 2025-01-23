/*
Solution: Brute Force

Time: O(m * n) | Space: O(m + n)

Runtime: 0 ms | 100.00%
Memory: 8.75 MB | 21.7480.00%

- m: rows of 'grid'
- n: columns of 'grid[0]'
*/

func countServers(grid [][]int) int {
	m := len(grid)
	n := len(grid[0])

	rows := make([]int, m, m)
	cols := make([]int, n, n)

	res := 0

	for i := range m {
		for j := range n {
			rows[i] += grid[i][j]
			cols[j] += grid[i][j]
		}

		res += rows[i]
	}

	for i := range m {
		for j := range n {
			if rows[i]*cols[j] == 1 && grid[i][j] == 1 {
				res -= 1
			}
		}
	}

	return res
}