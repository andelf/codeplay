#include <vector>
#include <algorithm>
#include <iostream>

using namespace std;


class Solution {
public:
    bool isMatch(string s, string p) {



      return false;
    }

};


int main(int argc, char *argv[])
{
  auto s = Solution{};

  cout << s.isMatch("aa", "a") << '\n';
  cout << s.isMatch("aa", "aa") << endl;
  cout << s.isMatch("aa", "a*") << endl;
  return 0;
}
