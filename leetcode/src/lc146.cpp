/*
 * Design and implement a data structure for Least Recently Used (LRU) cache.
 * It should support the following operations: get and set.
 *
 * get(key) - Get the value (will always be positive) of the key if the key
 * exists in the cache, otherwise return -1.
 *
 * set(key, value) - Set or insert the value if the key is not already present.
 * When the cache reached its capacity, it should invalidate the least recently
 * used item before inserting a new item.
 */

#include <iostream>
#include <string>
#include <array>
#include <vector>
#include <deque>
#include <list>
#include <set>
#include <map>
#include <unordered_set>
#include <unordered_map>
#include <stack>
#include <queue>
#include <algorithm>
#include <numeric>
#include <cmath>
#include <cassert>


using namespace std;


class LRUCache{
    map<int, int> timeline;  // tick -> key
    unordered_map<int, int> rev_timeline;    // key -> newest_tick
    unordered_map<int, int> cache;  // key -> val
    int cap;
    int tick;
public:
    LRUCache(int capacity) {
        cap = capacity;
        tick = 0;
    }

    int get(int key) {
        auto it = cache.find(key);
        if (it != cache.end()) {
            timeline.insert( {++tick, key} );
            auto last_access = rev_timeline[key];
            timeline.erase(timeline.find(last_access));
            rev_timeline[key] =  tick;
            return it->second;
        }
        return -1;
    }

    void set(int key, int value) {
        timeline.insert( {++tick, key} );
        auto it = cache.find(key);
        if (it != cache.end()) {
            auto last_access = rev_timeline[key];
            timeline.erase(timeline.find(last_access));
            rev_timeline[key] = tick;
            it->second = value;
        } else {
            rev_timeline[key] = tick;
            if (cache.size() == cap) {
                auto lru_key = timeline.begin()->second;
                cache.erase(cache.find(lru_key));
                rev_timeline.erase(rev_timeline.find(lru_key));
                timeline.erase(timeline.begin());
                cache[key] = value;
                assert(cache.size() <= cap);
            } else {
                cache[key] = value;
            }
        }
    }
};

int main(int argc, char *argv[])
{
    LRUCache cache(2);
    cache.set(2, 1);
    assert(cache.get(2) == 1);
    return 0;
}
