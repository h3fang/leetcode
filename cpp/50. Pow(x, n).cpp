#include <cmath>
#include <cstdio>
#include <cstdint>

using namespace std;

class Solution {
public:
    double myPow(double x, int n) {
        if (n == 0) {
            return 1.0;
        }
        if (n == INT32_MIN) {
            x *= x;
            n /= 2;
        }
        if (n < 0) {
            x = 1/x;
            n = -n;
        }
        return n%2 == 0 ? myPow(x*x, n/2) : x * myPow(x*x, n/2);
    }
};

int main() {
    printf("%.5f\n", Solution().myPow(1.01, INT32_MIN));
    return 0;
}