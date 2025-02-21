/*
Solution: DFS

Time: O(n) | Space: O(n)

Runtime: 2 ms | 100.00%
Memory: 8.91 MB | 94.44%

- n: # of nodes in 'root'
*/

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
type FindElements struct {
	values map[int]bool
}

func Constructor(root *TreeNode) FindElements {
	set := make(map[int]bool)
	dfs(&set, 0, root)

	return FindElements{values: set}
}

func (this *FindElements) Find(target int) bool {
	return this.values[target]
}

func dfs(set *map[int]bool, value int, root *TreeNode) {
	if root != nil {
		(*set)[value] = true
		dfs(set, value*2+1, root.Left)
		dfs(set, value*2+2, root.Right)
	}
}

/**
 * Your FindElements object will be instantiated and called as such:
 * obj := Constructor(root);
 * param_1 := obj.Find(target);
 */