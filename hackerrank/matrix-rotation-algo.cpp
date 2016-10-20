#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;





int main() {
    /* Enter your code here. Read input from STDIN. Print output to STDOUT */
    int m, n, r;
    cin >> m >> n >> r;
    int nloops = min(m, n) / 2;

    vector<vector<int>> mat(m+1, vector<int>(n+1, 0));
    vector<vector<int>> mat2(mat);

    for (auto i=1; i <= m; ++i) {
        for (auto j=1; j <= n; ++j) {
            int tmp;
            cin >> tmp;
            mat[i][j] = tmp;
        }
    }

    for (auto k=0; k < nloops; ++k) {
        int h = m-k*2;
        int w = n-k*2;
        int n_items = h*2 + w*2 - 4;
        int rotate = r % n_items;

/*        if (rotate == 0) {
          continue;
        } */

        int x0 = 1+k;
        int y0 = 1+k;

        int tmp = mat[y0][x0];

        for (auto i=0; i<n_items; ++i) {
            int x1 = x0;
            int y1 = y0;
            if (i < h) {
                x1 = x0;
                y1 = y0+i;
            } else if (i < h + w - 1) {
                x1 = x0 + i - h + 1;
                y1 = y0 + h -1;
            } else if (i < h*2 + w - 2) {
                x1 = x0+w-1;
                y1 = y0 + (h*2+w-2 -i) -1;
            } else {
                x1 = x0 + n_items - i;
                y1 = y0;
            }


            auto j = (i + rotate) % n_items;
            int x2 = x0;
            int y2 = y0;

            if (j < h) {
                x2 = x0;
                y2 = y0+j;
            } else if (j < h + w - 1) {
                x2 = x0 + j - h + 1;
                y2 = y0 + h -1;
            } else if (j < h*2 + w - 2) {
                x2 = x0+w-1;
                y2 = y0 + (h*2+w-2 -j) -1;
            } else {
                x2 = x0 + n_items - j;
                y2 = y0;
            }


            mat2[y2][x2] = mat[y1][x1];
        }


    }

    for (auto i=1; i <= m; ++i) {
        for (auto j=1; j <= n; ++j) {
            int tmp;
            //  cin >> tmp;
            cout << mat2[i][j] << ' ';
        }
        cout << endl;
    }





    return 0;
}
