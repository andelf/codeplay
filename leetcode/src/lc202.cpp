// Write an algorithm to determine if a number is "happy".

// A happy number is a number defined by the following process:
// Starting with any positive integer, replace the number by the
// sum of the squares of its digits, and repeat the process until
// the number equals 1 (where it will stay), or it loops endlessly
// in a cycle which does not include 1. Those numbers for which
// this process ends in 1 are happy numbers.


#include <algorithm>
#include <unordered_set>
#include <iostream>


using namespace std;


class Solution {
public:
  bool isHappy(int n) {
    unordered_set<int> sums;
    while (n != 1) {
      auto sum = 0;
      for (auto v = n; v != 0; v = v/10) {
        sum += (v % 10) * (v % 10);
      }
      // cout << sum << '\n';
      if (sums.find(sum) != sums.cend()) {
        return false;
      }
      if (sum == 1) {
        return true;
      }
      sums.insert(sum);
      n = sum;
    }
    return true;
  }
};


int main(int argc, char *argv[])
{
  cout << Solution().isHappy(7) << endl;
  return 0;
}
