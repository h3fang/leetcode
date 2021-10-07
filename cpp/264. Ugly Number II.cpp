#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int nthUglyNumber(int n) {
        vector<int> ugly{1};
        int f2 = 0, f3 = 0, f5 = 0;
        for (int i = 1; i < n; i++) {
            int m = min(2*ugly[f2], min(3*ugly[f3], 5*ugly[f5]));
            ugly.push_back(m);
            if (2*ugly[f2] == m) { f2++; }
            if (3*ugly[f3] == m) { f3++; }
            if (5*ugly[f5] == m) { f5++; }
        }

        return ugly[n - 1];
    }
};

int main() {
    printf("%d\n", Solution().nthUglyNumber(1690));
    return 0;
}