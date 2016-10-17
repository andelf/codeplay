#include <iostream>
#include <string>
// #include <array>
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

using namespace std;


class MedianFinder {
public:
    vector<int> maxHeap;        // smaller
    vector<int> minHeap; // all negative values, bigger
    // Adds a number into the data structure.
    void addNum(int num) {
        if (maxHeap.empty() || num < maxHeap[0]) {
            maxHeap.push_back(num);
            push_heap(maxHeap.begin(), maxHeap.end());
        } else {
            minHeap.push_back(-num);
            push_heap(minHeap.begin(), minHeap.end());
        }

        // adjust
        if (minHeap.size() > maxHeap.size()) {
            pop_heap(minHeap.begin(), minHeap.end());
            int val = -minHeap.back();
            minHeap.pop_back();

            maxHeap.push_back(val);
            push_heap(maxHeap.begin(), maxHeap.end());
        }
        if (maxHeap.size() > minHeap.size() + 1) {
            pop_heap(maxHeap.begin(), maxHeap.end());
            auto val = maxHeap.back();
            maxHeap.pop_back();

            minHeap.push_back(-val);
            push_heap(minHeap.begin(), minHeap.end());
        }
    }

    // Returns the median of current data stream
    double findMedian() {
        if (maxHeap.size() == minHeap.size()) {
            return (maxHeap[0] - minHeap[0]) / 2.0;
        } else {
            return maxHeap[0];
        }
    }
};

// Your MedianFinder object will be instantiated and called as such:
// MedianFinder mf;
// mf.addNum(1);
// mf.findMedian();

int main(int argc, char *argv[])
{
  MedianFinder mf;
  mf.addNum(1);
  mf.addNum(2);
  cout << mf.findMedian() << endl;
  mf.addNum(3);
  cout << mf.findMedian() << endl;
  return 0;
}
