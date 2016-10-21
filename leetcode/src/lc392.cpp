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
    bool isSubsequence(string s, string t) {
        if (s.size() > t.size()) {
            return false;
        }
        if (s.size() == t.size()) {
            return s == t;
        }
        if (s.size() == 0) {
            return true;
        }

        if (s.back() == t.back()) {
            if (isSubsequence(s.substr(0, s.size()-1), t.substr(0, t.size()-1))) {
                return true;
            }
            return isSubsequence(s, t.substr(0, t.size()-1));
        } else {
            return isSubsequence(s, t.substr(0, t.size()-1));
        }
        return false;
    }
};


int main(int argc, char *argv[])
{
    Solution s;
    assert(  s.isSubsequence("abc", "ahbgdc"));
    assert(! s.isSubsequence("axc", "ahbgdc"));
    return 0;
}
