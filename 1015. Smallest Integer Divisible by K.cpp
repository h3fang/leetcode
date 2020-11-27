#include <cstdio>

using namespace std;

class Solution {
public:
    int smallestRepunitDivByK(int K) {
        int r = 0;
        for (int i = 1; i < K+1; i++) {
            r = (r * 10 + 1) % K;
            if (r == 0) {
                return i;
            }
        }
        return -1;
    }
};

int main() {
    printf("%d\n", Solution().smallestRepunitDivByK(11));
    return 0;
}
