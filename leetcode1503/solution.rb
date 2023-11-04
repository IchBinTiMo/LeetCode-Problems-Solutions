# @param {Integer} n
# @param {Integer[]} left
# @param {Integer[]} right
# @return {Integer}
def get_last_moment(n, left, right)
    if (if left.empty? then 0 else left.max end) > (if right.empty? then 0 else n - right.min end) then (if left.empty? then 0 else left.max end) else (if right.empty? then 0 else n - right.min end) end
end