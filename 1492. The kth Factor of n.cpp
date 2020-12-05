#include <cstdio>
#include <vector>
#include <cmath>

using namespace std;

class Solution {
public:
    int kthFactor(int n, int k) {
        float root = sqrt(float(n));
        for (int i = 1; i < root; i++) {
            if (n % i == 0 && --k == 0) {
                return i;
            }
        }

        for (int i = root; i>=1; i--) {
            if (n % i == 0 && --k == 0) {
                return n / i;
            }
        }
        return -1;
    }
};

int main() {
    printf("%d\n", Solution().kthFactor(12, 3));
    return 0;
}
