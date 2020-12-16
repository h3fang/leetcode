#include <cstdio>
#include <cmath>

using namespace std;

class Solution {
public:
    int poorPigs(int buckets, int minutesToDie, int minutesToTest) {
        int T = minutesToTest / minutesToDie;
        return int(ceil(log(buckets) / log(T + 1)));
    }
};

int main() {
    printf("%d\n", Solution().poorPigs(1000, 15, 60));
    return 0;
}