#include <vector>
#include <algorithm>
#include <iostream>

using namespace std;


class Solution {
public:
    bool isMatch(string s, string p) {
        auto lenS = s.size();
        auto lenP = p.size();

        if (lenP == 0) {
            return lenS == 0;
        }

        if (lenP == 1 || p[1] != '*' ) {
            if (lenS > 0 && (p[0] == '.' || s[0] == p[0]))
                return isMatch(s.substr(1), p.substr(1));
            return false;
        } else {
            if (lenS == 0) return isMatch(s, p.substr(2));
            if (isMatch(s, p.substr(2))) return true;
            for (auto i=0; i< lenS; ++i) {
                // // `.*` matches any duplicated
                // if ((s[i] == s[0]) && (s[0] == p[0] || p[0] == '.')) {
                if ((s[i] == s[0] && s[0] == p[0]) || p[0] == '.') {
                    if (isMatch(s.substr(i+1), p.substr(2))) {
                        return true;
                    }
                } else {
                    return false;
                }
            }
        }
        return false;
    }
};


int main(int argc, char *argv[])
{
    auto s = Solution{};

    cout << s.isMatch("aa", "a") << '\n';
    cout << s.isMatch("aa", "aa") << endl;
    cout << s.isMatch("aa", "a*") << endl;
    cout << s.isMatch("a", ".*..a*") << endl;
    cout << s.isMatch("", ".") << endl;
    cout << s.isMatch("aa", ".*") << endl;
    cout << s.isMatch("ab", ".*") << endl;
    return 0;
}
