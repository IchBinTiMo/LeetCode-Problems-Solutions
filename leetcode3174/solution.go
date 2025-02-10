/*
Solution: String

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 4.05 MB | 93.33%

- n: length of 's'
*/

func clearDigits(s string) string {
	var stack []rune

	for _, c := range s {
		if c-'0' <= 9 && stack[len(stack)-1]-'0' > 9 {
			stack = stack[:len(stack)-1]
		} else {
			stack = append(stack, c)
		}
	}

	return string(stack)
}