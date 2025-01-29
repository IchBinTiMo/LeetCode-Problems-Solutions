/*
Solution: union find

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 5.00 MB | 42.03%

- n: length of nodes
*/

func findRedundantConnection(edges [][]int) []int {
	n := len(edges)

	var parent []int
	var degree []int

	for i := range n + 1 {
		parent = append(parent, i)
		degree = append(degree, 1)
	}

	var res []int

	for _, edge := range edges {
		u := edge[0]
		v := edge[1]

		if !union(u, v, parent, degree) {
			res = append(res, u)
			res = append(res, v)

			return res
		}
	}

	return res
}

func find(node int, parent []int) int {
	p := parent[node]

	for p != parent[p] {
		p = parent[p]
	}

	return p
}

func union(u int, v int, parent []int, degree []int) bool {
	parent_u := find(u, parent)
	parent_v := find(v, parent)

	if parent_u == parent_v {
		return false
	}

	if degree[parent_u] > degree[parent_v] {
		parent[parent_v] = parent_u
		degree[parent_u] += degree[parent_v]
	} else {
		parent[parent_u] = parent_v
		degree[parent_v] += degree[parent_u]
	}

	return true
}