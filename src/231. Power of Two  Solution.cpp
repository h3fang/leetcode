#include <cstdio>
#include <cmath>
#include <vector>

using namespace std;

class Solution {
public:
    bool isPowerOfTwo(int n) {
        return n <= 0 ? false : __builtin_popcount(n) == 1;
    }
};

int main() {
    const int n = 100;
    printf("%d\n", Solution().isPowerOfTwo(n));
    return 0;
}