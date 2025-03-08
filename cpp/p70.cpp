#include <cassert>

using namespace std;

class Solution {
public:
    int climbStairs(int n) {
        int f0 = 1, f1 = 1;
        for (int i = 2; i <= n; i++) {
            int t = f1;
            f1 += f0;
            f0 = t;
        }
        return f1;
    }
};

int main() {
    assert(3, Solution().climbStairs(3));
    assert(1836311903, Solution().climbStairs(45));
    return 0;
}