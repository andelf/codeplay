// https://leetcode.com/problems/merge-intervals/

/**
 * Definition for an interval.
 * struct Interval {
 *     int start;
 *     int end;
 *     Interval() : start(0), end(0) {}
 *     Interval(int s, int e) : start(s), end(e) {}
 * };
 */
class Solution {
public:
  vector<Interval> merge(vector<Interval>& intervals) {
    // 1 start -1 ends
    auto Cmp = [](Interval &a, Interval &b) -> bool {
      return a.start < b.start;
    };

    vector<Interval> ret;

    map<int, int> timeline;
    for (auto it=intervals.cbegin(); it!= intervals.cend(); ++it) {
      if (timeline.count(it->start) != 0) {
        timeline[it->start] += 1;
      } else {
        timeline[it->start] =  1;
      }

      if (timeline.count(it->end) != 0) {
        timeline[it->end] -= 1;  // ends
      } else {
        timeline[it->end] = -1;
      }
    }

    int current_time = -1;
    int current_state = 0;
    for (auto it=timeline.cbegin(); it != timeline.cend(); ++it) {
      if (current_state == 0 && it->second > 0) {
        current_time = it->first;  // update interval start
      }

      if (it->second != 0) {
        current_state += it->second;
        if (current_time >= 0 && current_state == 0) {
          ret.push_back({current_time, it->first});
        }
      } else if (current_state == 0) {
        ret.push_back({it->first, it->first});  // [0, 0]
      }
    }
    return ret;
  }
};
