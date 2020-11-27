#include <cstdio>

using namespace std;

class Solution {
public:
    int mirrorReflection(int p, int q) {
        int m = 1, n = 1;
        while(m * p != n * q) {
            n++;
            m = n * q / p;
        }
        if (n % 2 == 0) {
            return 2;
        }
        else {
            if (m % 2 == 0) {
                return 0;
            }
            else {
                return 1;
            }
        }
        return 0;
    }
};

int main() {
    printf("%d\n", Solution().mirrorReflection(3, 1));
    return 0;
}
