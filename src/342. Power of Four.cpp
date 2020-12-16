#include <cstdio>

using namespace std;

class Solution {
public:
    bool isPowerOfFour(int num) {
        long n = num;
        return (n & (n-1)) == 0 && n & 0b01010101010101010101010101010101;
    }
};

int main() {
    printf("%d\n", Solution().isPowerOfFour(255));
    return 0;
}