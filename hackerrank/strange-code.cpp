#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;


int main() {
    /* Enter your code here. Read input from STDIN. Print output to STDOUT */
    long long t;
    cin >> t;
    double sum = 0.0;
    for (double i=0.0; true; i+=1) {
        double cycle = 3 * pow(2, i);
        if (sum + cycle + 0.1 > t) {
            double val = cycle - (t - sum) + 1;
            printf("%.0lf\n", val);
            return 0;
        } else {
            sum += cycle;
        }
    }
    return 0;
}
