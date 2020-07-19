#include <cmath>
#include <cstdio>

using namespace std;

class Solution {
public:
    double angleClock(int hour, int minutes) {
        double m = minutes * 6;
        double h = hour * 30 + minutes * 0.5;
        double a = abs(m - h);
        return min(a, 360.0 - a);
    }
};

int main() {
    printf("%.5f\n", Solution().angleClock(4, 51));
    return 0;
}