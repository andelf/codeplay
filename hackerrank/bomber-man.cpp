#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;


int main() {
    /* Enter your code here. Read input from STDIN. Print output to STDOUT */
    int R, C, N;
    cin >> R >> C >> N;
    const int EMPTY=0;
    const int BOOM0=1;
    const int BOOM1=2;
    const int BOOM2=3;  // boomed


    vector<vector<int>> grid(R, vector<int>(C, EMPTY));

    for (auto j=0; j<R; ++j) {
        string line;
        cin >> line;
        for (auto i=0; i<C; ++i) {
            if (line[i] == 'O') {
                grid[j][i] = BOOM0;
            }

        }
    }


    int round = 0;
    while (--N) {
        round++;
        if (round % 2 == 0) {
            for (auto j=0; j<R; ++j) {
                for (auto i=0; i<C; ++i) {
                    if (grid[j][i] == BOOM1) {
                        grid[j][i] = BOOM2;
                    } else if (j < R-1 && grid[j+1][i] >= BOOM1) {
                        grid[j][i] = EMPTY;
                    } else if (i < C-1 && grid[j][i+1] >= BOOM1) {
                        grid[j][i] = EMPTY;
                    } else if (j > 0 && grid[j-1][i] >= BOOM1) {
                        grid[j][i] = EMPTY;
                    } else if (i > 0 && grid[j][i-1] >= BOOM1) {
                        grid[j][i] = EMPTY;
                    }
                }
            }



        } else {
            for (auto j=0; j<R; ++j) {
                for (auto i=0; i<C; ++i) {
                    if (grid[j][i] == EMPTY || grid[j][i] == BOOM2) {
                        grid[j][i] = BOOM0;
                    } else if (grid[j][i] == BOOM0) {
                        grid[j][i] = BOOM1;
                    }
                }
            }
        }
    }

    for (auto j=0; j<R; ++j) {
        for (auto i=0; i<C; ++i) {
            if (grid[j][i] == EMPTY || grid[j][i] == BOOM2) {
                cout << '.';
            } else if (grid[j][i] == BOOM1 || grid[j][i] == BOOM0) {
                cout << 'O';
            } else {
                cout << grid[j][i];
            }
        }
        cout << endl;
    }

    return 0;
}
