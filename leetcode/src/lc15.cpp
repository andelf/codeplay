#include <iostream>
#include <cstdio>
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
    vector<vector<int>> threeSum(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        set<vector<int>> result;

        for (auto it1=nums.begin(); it1 != nums.end(); ++it1) {
            for (auto it2=next(it1); it2 != nums.end(); ++it2) {
                auto val = 0 - *it1 - *it2;
                for (auto search = lower_bound(next(it2), nums.end(), val);
                        search != nums.end() && *search == val;
                        ++search) {

                    result.insert( {*it1, *it2, *search } );
                    break;

                }
            }
        }
        // result.erase(unique(result.begin(), result.end()), result.end());
				vector<vector<int>> ret(result.begin(), result.end());
        return ret;

    }
};


// FIXME: timeout
int main(int argc, char *argv[])
{
    vector<int> nums = {7,-1,14,-12,-8,7,2,-15,8,8,-8,-14,-4,-5,7,9,11,-4,-15,-6,1,-14,4,3,10,-5,2,1,6,11,2,-2,-5,-7,-6,2,-15,11,-6,8,-4,2,1,-1,4,-6,-15,1,5,-15,10,14,9,-8,-6,4,-6,11,12,-15,7,-1,-9,9,-1,0,-4,-1,-12,-2,14,-9,7,0,-3,-4,1,-2,12,14,-10,0,5,14,-1,14,3,8,10,-8,8,-5,-2,6,-11,12,13,-7,-12,8,6,-13,14,-2,-5,-11,1,3,-6};

    Solution s;
    auto result = s.threeSum(nums);

    for_each(result.cbegin(), result.cend(), [](const vector<int> &tup) {
						printf("[%d, %d, %d]\n", tup[0], tup[1], tup[2]);
				});
    return 0;
}
