#include <set>
#include <queue>
#include <map>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> countBits(int num) {
        vector<int> r(num+1);
        for (int i=1; i<=num; i++) {
            r[i] = r[i/2] + i%2;
        }
        return r;
    }
};

int main() {
    const int N = 64;
    for (int i : Solution().countBits(N)) {
        printf("%d ", i);
    }
    printf("\n");
    return 0;
}