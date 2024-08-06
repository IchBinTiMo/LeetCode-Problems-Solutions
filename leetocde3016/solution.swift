/*
Solution 1:

Time: O(n) | Space: O(n)

- n: length of `word`
*/

class Solution {
    func minimumPushes(_ word: String) -> Int {
        var freqs: Array<Int> = Array(repeating: 0, count: 26);

        for c in word {
            freqs[Int(c.asciiValue!) - 97] += 1;
        }

        freqs.sort();

        var res: Int = 0;
        var mapped: Int = 0;

        while let freq = freqs.popLast() {
            res += (mapped / 8 + 1) * freq;
            mapped += 1;
        }        

        return res;
    }
}