/*
Solution: Sweep Line

Time: O(n ^ 2) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 6.9 MB | 100.00%

- n: # of operations called on 'MyCalendarTwo'
*/

class MyCalendarTwo {
public:
    map<int, int> line;

    MyCalendarTwo() {}
    
    bool book(int start, int end) {

        line[start] += 1;
        line[end] -= 1;

        int cnt = 0;

        for (auto& entry: line) {
            cnt += entry.second;

            if (cnt >= 3) {
                line[start] -= 1;
                line[end] += 1;

                return false;
            }
        }

        return true;
    }
};

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * MyCalendarTwo* obj = new MyCalendarTwo();
 * bool param_1 = obj->book(start,end);
 */