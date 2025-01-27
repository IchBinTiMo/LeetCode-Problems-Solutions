/*
solution: Topological Sort

Time: O(n) | Space: O(n * n)

Runtime: 0 ms | 100.00%
Memory: 9.02 MB | 96.00%

- n: length of 'num_courses'
*/

func checkIfPrerequisite(numCourses int, prerequisites [][]int, queries [][]int) []bool {
	prevs := make([][2]int, numCourses)
	in_degree := make([]int, numCourses)
	adj := make([][]int, numCourses)

	for _, pre := range prerequisites {
		u := pre[0]
		v := pre[1]

		adj[u] = append(adj[u], v)
		in_degree[v] += 1
	}

	var q []int

	for i := range numCourses {
		if in_degree[i] == 0 {
			q = append(q, i)
		}
	}

	for len(q) > 0 {
		current := q[0]
		q = q[1:]

		for len(adj[current]) > 0 {
			next := adj[current][0]
			adj[current] = adj[current][1:]

			if current <= 63 {
				prevs[next][1] |= 1 << current
			} else {
				prevs[next][0] |= 1 << (current - 64)
			}

			for i := range 2 {
				prevs[next][i] |= prevs[current][i]
			}

			in_degree[next] -= 1

			if in_degree[next] == 0 {
				q = append(q, next)
			}
		}
	}

	var res []bool

	for _, query := range queries {
		u := query[0]
		v := query[1]

		if u <= 63 {
			res = append(res, (prevs[v][1]&(1<<u)) != 0)
		} else {
			res = append(res, (prevs[v][0]&(1<<(u-64))) != 0)
		}
	}

	return res
}