#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
#include <limits>

using namespace std;


int main() {
    int T;
    cin >> T;
    while (T--) {
        int N;
        cin >> N;

        N--;
        int tmp;
        cin >> tmp;

        int max_ending_here = tmp;
        int max_so_far = tmp;

        int max_val = tmp;
        int sum_val = 0;
        if (tmp > 0)
          sum_val += tmp;


        for (auto i=0; i<N; ++i) {
            cin >> tmp;
            max_ending_here = max(tmp, max_ending_here + tmp);
            max_so_far = max(max_ending_here, max_so_far);

            max_val = max(max_val, tmp);
            if (tmp > 0) {
                sum_val += tmp;
            }
        }

        cout << max_so_far << ' ';

        if (sum_val > 0) {
            cout << sum_val << endl;
        } else {
            cout << max_val << endl;
        }
    }
    return 0;
}
