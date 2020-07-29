#include <cstdio>

using namespace std;

class Solution {
public:
    int addDigits(int num) {
        int n = num;
        while (n>=10) {
            int r = 0;
            while (n>0) {
                r += n % 10;
                n /= 10;
            }
            n = r;
        }
        return n;
    }
};

int main() {
    printf("%d\n", Solution().addDigits(38));
    return 0;
}