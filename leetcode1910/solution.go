/*
Solution: Stack

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 4.29 MB | 100.00%

- n: length of 's'
*/

func removeOccurrences(s string, part string) string {

	index := strings.Index(s, part)
	for index != -1 {
		s = s[:index] + s[index+len(part):]
		index = strings.Index(s, part)
	}
	return s
}

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