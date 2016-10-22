// Maximum Subarray
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
#define BOOST_TEST_MODULE Maximum Subarray
#include <boost/test/included/unit_test.hpp>

using namespace std;


class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int max_ending_here = nums[0];
        int max_so_far = nums[0];
        for (auto it=next(nums.cbegin()); it!=nums.cend(); ++it) {
            max_ending_here = max(*it, max_ending_here + *it);
            max_so_far = max(max_ending_here, max_so_far);
        }
        return max_so_far;

    }
};


BOOST_AUTO_TEST_CASE( my_test )
{
    Solution s;
    vector<int> coll = {-2,1,-3,4,-1,2,1,-5,4};
    BOOST_TEST( 6 == s.maxSubArray( coll ));
}
