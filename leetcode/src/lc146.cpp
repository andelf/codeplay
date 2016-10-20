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
    list<pair<int, int>> cache;
    int cap;
public:
    LRUCache(int capacity) {
        cap = capacity;
    }

    int get(int key) {
        auto it = find_if(cache.begin(), cache.end(), [&key](const pair<int,int> &kv) {
            return kv.first == key;
        });
        if (it != cache.end()) {
            auto kv = *it;
            cache.erase(it);
            cache.push_front(kv);
            return kv.second;
        }
        return -1;
    }

    void set(int key, int value) {
        if (get(key) != -1) {
            cache.front().second = value;
        } else {
            if (cache.size() == cap) {
                cache.pop_back();
                cache.push_front( {key, value});
            } else {
                cache.push_front( {key, value});
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
