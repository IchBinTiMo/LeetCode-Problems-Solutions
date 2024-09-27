/*
Solution 1: Brute Force

Time: O(n ^ 2) | Space: O(n)

Runtime: 57 ms | 98.85%
Memory: 37.86 MB | 96.56%

- n: # of operations called on 'MyCalendarTwo'
*/

class MyCalendarTwo {
    vector<pair<int,int>> b;
    vector<pair<int,int>>db;
public:
    MyCalendarTwo() {
        
    }
    
    bool book(int start, int end) {
        
        for(pair<int,int> x: db){
            if(start<x.second && end>x.first) return false;
        }
        for(pair<int,int> x : b){
            if(start<x.second && end>x.first){
                db.push_back({max(start,x.first),min(end,x.second)});
            }
        }
        b.push_back({start,end});
        return true;
        
    }
};

/*
Solution 2: Sweep Line

Time: O(n ^ 2) | Space: O(n)

Runtime: 151 ms | 71.11%
Memory: 42.66 MB | 46.51%

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