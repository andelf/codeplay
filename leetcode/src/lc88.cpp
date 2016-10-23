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


class Solution {
public:
    void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
        vector<int> res;
        auto it1 = nums1.begin();
        auto it2 = nums2.begin();

        while(it1 != nums1.begin()+m && it2 != nums2.begin()+n) {
            if (*it1 <= *it2) {
                res.push_back(*it1++);
            } else {
                res.push_back(*it2++);
            }
        }
        while (it1 != nums1.begin()+m) {
            res.push_back(*it1++);
        }
        while (it2 != nums2.begin()+n) {
            res.push_back(*it2++);
        }
        nums1.assign(res.begin(), res.begin()+m+n);
    }
};


int main(int argc, char *argv[])
{

    return 0;
}
