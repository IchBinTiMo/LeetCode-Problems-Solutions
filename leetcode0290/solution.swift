class Solution {
    func wordPattern(_ pattern: String, _ s: String) -> Bool {
        /*
        Time: O(n) | Space: O(n)

        where n is the length of `pattern`
        */
        var map = [String: String]();
        var map2 = [String: String]();

        var strs = s.components(separatedBy: " ");

        if (pattern.count != strs.count) {
            return false;
        }

        for (i, c) in pattern.enumerated() {
            var c = String(c);
            var str = strs[i];

            if (map.keys.contains(c) || map2.keys.contains(str)) {
                if (str != map[c]) {
                    return false;
                }
                if (c != map2[str]) {
                    return false;
                }
            } else {
                map[c] = str;
                map2[str] = c;
            }
        }
        
        return true;
    }
}