class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []

        current = 0

        operators = ["+", "-", "*", "/"]

        while current < len(tokens):
            if tokens[current] not in operators:
                stack.append(int(tokens[current]))
            else:
                v2 = stack.pop()
                v1 = stack.pop()
                match tokens[current]:
                    case "+":
                        stack.append(v1 + v2)
                    case "-":
                        stack.append(v1 - v2)
                    case "*":
                        stack.append(v1 * v2)
                    case "/":
                        stack.append(int(v1 / v2))
                    case _:
                        pass
            current += 1


        return stack[0]

# class Solution:
#     def evalRPN(self, tokens: List[str]) -> int:
#         stack = []

#         current = 0

#         while current < len(tokens):
#             if self.is_a_number(tokens[current]):
#                 stack.append(int(tokens[current]))
#             else:
#                 v2 = stack.pop()
#                 v1 = stack.pop()
#                 match tokens[current]:
#                     case "+":
#                         stack.append(v1 + v2)
#                     case "-":
#                         stack.append(v1 - v2)
#                     case "*":
#                         stack.append(v1 * v2)
#                     case "/":
#                         stack.append(int(v1 / v2))
#                     case _:
#                         pass
#             current += 1


#         return stack[0]

#     def is_a_number(self, x):
#         try:
#             int(x)
#             return True
#         except ValueError:
#             return False