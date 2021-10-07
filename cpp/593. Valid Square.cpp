#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    bool isITOrdered(const vector<int>& p1, const vector<int>& p2, const vector<int>& p3) {
        const int x1 = p2[0] - p1[0], x2 = p3[0] - p1[0];
        const int y1 = p2[1] - p1[1], y2 = p3[1] - p1[1];
        const int l1 = x1 * x1 + y1 * y1, l2 = x2 * x2 + y2 * y2;
        return (x1 * x2 + y1 * y2) == 0 && l1 > 0 && l1 == l2;
    }

    bool isIT(const vector<int>& p1, const vector<int>& p2, const vector<int>& p3) {
        return isITOrdered(p1, p2, p3) || isITOrdered(p2, p1, p3) || isITOrdered(p3, p1, p2);
    }

    bool validSquare(vector<int>& p1, vector<int>& p2, vector<int>& p3, vector<int>& p4) {
        return isIT(p1, p2, p3) &&
        isIT(p4, p2, p3) &&
        isIT(p1, p2, p4) &&
        isIT(p1, p4, p3);
    }
};

int main() {
    vector<int> p1 = {0,0}, p2 = {1,1}, p3 = {1,0}, p4 = {0,1};
    printf("%s\n", Solution().validSquare(p1, p2, p3, p4) ? "true" : "false");
    return 0;
}
