/*
Solution: Recursion

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 4.63 MB | 18.18%

- n: length of 'expression'
*/

func parseBoolExpr(expression string) bool {
	current := len(expression) - 1

	return recursive(expression, &current)
}

func recursive(exp string, current *int) bool {
	stack := []bool{}

	for *current >= 0 {
		switch exp[*current] {
		case 'f':
			stack = append(stack, false)
			*current -= 1
		case 't':
			stack = append(stack, true)
			*current -= 1
		case '|':
			res := stack[0]

			for i := 1; i < len(stack); i++ {
				res = res || stack[i]
			}

			return res
		case '&':
			res := stack[0]

			for i := 1; i < len(stack); i++ {
				res = res && stack[i]
			}

			return res
		case '!':
			res := stack[0]

			return !res
		case ')':
			*current -= 1
			stack = append(stack, recursive(exp, current))
			*current -= 1
		default:
			*current -= 1
		}
	}

	return stack[0]
}