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


/**
 * Definition for an interval.
 */
struct Interval {
    int start;
    int end;
    Interval() : start(0), end(0) {}
    Interval(int s, int e) : start(s), end(e) {}
};


class Solution {
public:
    vector<Interval> insert(vector<Interval>& intervals, Interval newInterval) {
        vector<Interval> ans(intervals);

        auto it = lower_bound(ans.begin(), ans.end(), newInterval, [](const Interval &a, const Interval &b) {
            return a.start < b.start;
        });



        if (it == ans.end()) {
            if (ans.size() > 0 && newInterval.start <= ans.back().end) {
                ans.back().end = max(newInterval.end, ans.back().end);
            } else {
                ans.push_back(newInterval);
            }
        } else {
            // overlap with previous
            if (it != ans.begin() && prev(it)->end >= newInterval.start) {
                --it;
            }

            // overlap with next
            if (newInterval.end >= it->start) {
                it->start = min(it->start, newInterval.start);
                it->end = max(it->end, newInterval.end);
                for (it=next(it); it!= ans.end(); ) {
                    if (it->start <= newInterval.end) {
                        prev(it)->end = max(it->end, newInterval.end);
                        ans.erase(it);
                    } else {
                        break;
                    }
                }

            } else {
                ans.insert(it, newInterval);
            }

        }

        return ans;
    }
};


int main(int argc, char *argv[])
{

    return 0;
}
