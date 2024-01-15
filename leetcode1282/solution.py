class Solution:
    def groupThePeople(self, groupSizes: List[int]) -> List[List[int]]:
        ans = []
        ht = {}

        for i in range(len(groupSizes)):
            if groupSizes[i] not in ht:
                ht[groupSizes[i]] = [i]
            else:
                ht[groupSizes[i]].insert(0, i)
            if len(ht[groupSizes[i]]) == groupSizes[i]:
                ans.insert(0, ht[groupSizes[i]])
                ht[groupSizes[i]] = []

        return ans