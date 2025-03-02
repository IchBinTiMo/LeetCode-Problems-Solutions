/*
Solution: Two Pointers, HashMap

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 6.66 MB | 7.14%

- n: length of 'nums1' + length of 'nums2'
*/

func mergeArrays(nums1 [][]int, nums2 [][]int) [][]int {
	left := 0
	right := 0

	records := make(map[int][]int)

	var res [][]int

	for left < len(nums1) || right < len(nums2) {
		id := -1
		val := -1

		if right >= len(nums2) ||
			(left < len(nums1) && nums1[left][0] <= nums2[right][0]) {
			id = nums1[left][0]
			val = nums1[left][1]
			left += 1
		} else {
			id = nums2[right][0]
			val = nums2[right][1]
			right += 1
		}

		if tmp, ok := records[id]; ok {
			fmt.Println(tmp)
			idx := tmp[1]

			tmp[0] += val
			res[idx][1] = tmp[0]
		} else {
			records[id] = []int{val, len(res)}
			res = append(res, []int{id, val})
		}
	}

	return res
}