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
#define BOOST_TEST_MODULE leetcode
#include <boost/test/included/unit_test.hpp>


using namespace std;

class Solution {
public:
    vector<string> fullJustify(vector<string>& words, int maxWidth) {
        vector<string> ret;
        vector<string> current;
        int current_length = 0;
        int spaces = 0;
        for (auto i=0; i< words.size(); ++i) {
            // assume every word < maxWidth
            if (words[i].size() + current_length + current.size() > maxWidth) {
                // output
                spaces = current.size() - 1;    // one more
                int more_spaces = 0;
                int remainder = 0;
                if (spaces != 0) {
                    more_spaces = (maxWidth - current_length) / spaces - 1;
                }
                remainder = maxWidth - current_length - spaces - more_spaces * spaces;

                string s;
                for (auto it=current.begin(); it!=current.end(); ++it) {
                    s += *it;
                    s += string(1 + more_spaces + bool(remainder--), ' ');
                    if (remainder < 0) {
                        remainder = 0;
                    }
                }
                if (remainder > 0) {
                    s += string(remainder, ' ');
                }

                ret.push_back(s.substr(0, maxWidth));
                current.clear();
                spaces = 0;
                current_length = 0;

            }
            current.push_back(words[i]);
            current_length += words[i].size();
        }
        // last one
        if (!current.empty()) {
            string s;
            int remainder = maxWidth - current_length - current.size() + 1;
            for (auto it=current.begin(); it!=current.end();  ++it) {
                s += *it;
                s += ' ';
            }
            // s.pop_back();
            s += string(remainder, ' ');
            ret.push_back(s.substr(0, maxWidth));
            current.clear();

        }
        return ret;
    }
};


BOOST_AUTO_TEST_CASE( basic_cases )
{
    Solution s;

    vector<string> case1 = {""};
    vector<string> result1 = s.fullJustify(case1, 0);
    vector<string> should1 {""};
    BOOST_REQUIRE_EQUAL_COLLECTIONS(result1.cbegin(), result1.cend(),
                                    should1.cbegin(), should1.cend());

    vector<string> case2 = {""};
    vector<string> result2 = s.fullJustify(case2, 2);
    vector<string> should2 {"  "};
    BOOST_REQUIRE_EQUAL_COLLECTIONS(result2.cbegin(), result2.cend(),
                                    should2.cbegin(), should2.cend());


    vector<string> case3 = {"a","b","c","d","e"};
    vector<string> result3 = s.fullJustify(case3, 1);
    vector<string> should3 {"a","b","c","d","e"};
    BOOST_REQUIRE_EQUAL_COLLECTIONS(result3.cbegin(), result3.cend(),
                                    should3.cbegin(), should3.cend());

    vector<string> case4 = {"a","b","c","d","e"};
    vector<string> result4 = s.fullJustify(case4, 3);
    vector<string> should4 {"a b","c d","e  "};
    BOOST_REQUIRE_EQUAL_COLLECTIONS(result4.cbegin(), result4.cend(),
                                    should4.cbegin(), should4.cend());

}

BOOST_AUTO_TEST_CASE( spaces )
{
    Solution s;

    vector<string> input = {"Listen","to","many,","speak","to","a","few."};
    vector<string> output = s.fullJustify(input, 6);
    vector<string> expected = {"Listen","to    ","many, ","speak ","to   a","few.  "};
    BOOST_REQUIRE_EQUAL_COLLECTIONS(output.cbegin(), output.cend(),
                                    expected.cbegin(), expected.cend());

}

BOOST_AUTO_TEST_CASE( wa_2 )
{
    Solution s;

    vector<string> input = {"Don't","go","around","saying","the","world","owes",
                            "you","a","living;","the","world","owes","you","nothing;",
                            "it","was","here","first."};
    vector<string> output = s.fullJustify(input, 30);
    vector<string> expected = {"Don't  go  around  saying  the",
                               "world  owes  you a living; the",
                               "world owes you nothing; it was",
                               "here first.                   "};
    // for (auto it=output.begin(); it != output.end(); ++it) {
    //     cout << '"' << *it << '"'<< endl;
    // }
    BOOST_REQUIRE_EQUAL_COLLECTIONS(output.cbegin(), output.cend(),
                                    expected.cbegin(), expected.cend());

}
