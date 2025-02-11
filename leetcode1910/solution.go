/*
Solution: Stack

Time: O(n) | Space: O(n)

Runtime: 3 ms | 43.24%
Memory: 8.44 MB | 29.73%

- n: length of 's'
*/

func removeOccurrences(s string, part string) string {
	n := len(part)

	var stack string

	for _, c := range s {
		stack += string(c)

		if len(stack) >= n && stack[(len(stack)-n):len(stack)] == part {
			stack = stack[0:(len(stack) - n)]
		}
	}

	return stack
}