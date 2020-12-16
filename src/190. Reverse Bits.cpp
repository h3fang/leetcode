#include <algorithm>
#include <bitset>
#include <cstdint>
#include <cstdio>

using namespace std;

class Solution {
public:
    uint32_t reverseBits(uint32_t n) {
        bitset<32> a = n;
        for (int i = 0; i < 16; i++) {
            bool b = a[i];
            a[i] = a[31 - i];
            a[31-i] = b;
        }

        return a.to_ulong();
    }
};

int main() {
    printf("%u\n", Solution().reverseBits(0b00000010100101000001111010011100));
    return 0;
}