class TimeMap {
public:
    TimeMap() {

    }
    
    void set(string key, string value, int timestamp) {
        db[key][timestamp] = value;
    }
    
    string get(string key, int timestamp) {
        auto it = db[key].upper_bound(timestamp);

        if (it == db[key].begin()) {
            return "";
        }

        --it;

        return it->second;

    }

private:
    unordered_map<string, map<int, string>> db;    
};

/**
 * Your TimeMap object will be instantiated and called as such:
 * TimeMap* obj = new TimeMap();
 * obj->set(key,value,timestamp);
 * string param_2 = obj->get(key,timestamp);
 */