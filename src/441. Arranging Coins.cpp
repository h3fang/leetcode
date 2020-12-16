#include <cstdio>
#include <cmath>
#include <vector>

using namespace std;

class Solution {
public:
    int arrangeCoins(int n) {
        return int(sqrt(2.0*n + 1.0/4) - 0.5);
    }
};

int main() {
    printf("%d\n", Solution().arrangeCoins(0));
    return 0;
}