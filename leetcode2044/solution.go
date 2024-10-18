/*
Solution: backtracking

Time: O(2^n) | Space: O(n)

Runtime: 5 ms | 86.21%
Memory: 3.88 MB | 21.74%

- n: length of 'nums'
*/

func countMaxOrSubsets(nums []int) int {
    mx := 0
    for _, num := range nums {
        mx |= num
    }

    res := 0;

    backtrack(&res, 0, mx, 0, nums);

    return res;
}

func backtrack(res *int, path int, mx int, current int, nums []int) {
    if current == len(nums) {
        if path == mx {
            *res += 1;
        }
        return;
    } else {
        backtrack(res, path, mx, current + 1, nums);
        backtrack(res, path | nums[current], mx, current + 1, nums);
    }
}