#include <cstdio>
#include <string>

using namespace std;

class Solution {
public:
    int maxPower(string s) {
        char c = 0;
        int n = 0, m = 1;
        for (auto e : s) {
            if (e == c) {
                n++;
            } else {
                m = std::max(n, m);
                n = 1;
                c = e;
            }
        }
        return std::max(n, m);
    }
};

int main() {
    printf("%d\n", Solution().maxPower("abbcccddddeeeeedcba"));
    return 0;
}