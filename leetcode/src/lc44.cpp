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
#define BOOST_TEST_MODULE Wildcard Matching
#include <boost/test/included/unit_test.hpp>

using namespace std;


class Solution {
public:
    bool isMatch(string s, string p) {
        auto lenS = s.size();
        auto lenP = p.size();
        vector<vector<bool>> dp {lenS+1, vector<bool>(lenP+1, false)};

        dp[0][0] = true;
        for (auto j = 1; j <= lenP; ++j) {
            if (p[j-1] == '*') {
                dp[0][j] = dp[0][j-1];
            }
        }

        for (auto i = 1; i <= lenS; ++i) {
            for (auto j = 1; j <= lenP; ++j) {
                if ( p[j-1] == '?' || p[j-1] == s[i-1]) { // match single
                    dp[i][j] = dp[i-1][j-1];
                } else if (p[j-1] == '*') {
                    dp[i][j] = dp[i][j-1] || dp[i-1][j-1] || dp[i-1][j];
                }
            }
        }

        // cout << s << '|' << p << endl;
        // for (auto row : dp) {
        //     for (auto val : row)
        //         cout << val << ' ';
        //     cout << endl;
        // }
        return dp[lenS][lenP];
    }
};



BOOST_AUTO_TEST_CASE( my_test )
{
    Solution s;

    BOOST_TEST( ! s.isMatch("aa",   "a")     );
    BOOST_TEST(   s.isMatch("aa",   "aa")    );
    BOOST_TEST( ! s.isMatch("aaa",  "aa")    );
    BOOST_TEST(   s.isMatch("aa",   "*")     );
    BOOST_TEST(   s.isMatch("aa",   "a*")    );
    BOOST_TEST(   s.isMatch("ab",   "?*")    );
    BOOST_TEST( ! s.isMatch("aab",  "c*a*b") );
}
