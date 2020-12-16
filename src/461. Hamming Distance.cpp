#include <cstdio>
#include <bitset>

using namespace std;

class Solution {
public:
    int hammingDistance(int x, int y) {
        bitset<sizeof(int)*8> n = x ^ y;
        return n.count();
    }
};

int main() {
    printf("%d\n", Solution().hammingDistance(1, 4));
    return 0;
}