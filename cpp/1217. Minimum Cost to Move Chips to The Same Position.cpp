#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int minCostToMoveChips(vector<int> &position) {
        int c = 0;
        for (int i : position) {
            if (i % 2 == 1) {
                c++;
            }
        }
        return min((int)position.size() - c, c);
    }
};

int main() {
    vector<int> input = {2, 2, 2, 3, 3};
    auto r = Solution().minCostToMoveChips(input);
    printf("%d\n", r);
    return 0;
}