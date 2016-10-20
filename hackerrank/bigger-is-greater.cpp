#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>

using namespace std;


int swap(string &s) {
    int length = s.size();
    for (auto j = length-1; j> 0; --j) {
        if (s[j-1] < s[j]) {
            // non increasing [j, i]
            int pivot = j-1;
            for (auto k=length-1; k>=j; --k) {
                if (s[k] > s[pivot]) { // rightmost > Pivot
                    char tmp = s[k]; //swap
                    s[k] = s[pivot];
                    s[pivot] = tmp;
                    reverse(s.begin()+j, s.end());
                    return 1;
                }
            }
        }
    }
    return 0;
}

int main() {
    /* Enter your code here. Read input from STDIN. Print output to STDOUT */
    int n;
    cin >> n;
    while (n--) {
        string s;
        cin >> s;
        if (s.size() == 1) {
            cout << "no answer" << endl;
        } else if (swap(s)) {
            cout << s << endl;
        } else {
            cout << "no answer" << endl;
        }

    }
    return 0;
}
