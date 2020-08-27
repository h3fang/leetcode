#include <vector>
#include <cstdio>

using namespace std;

class Solution {
public:
    vector<int> getRow(int rowIndex) {
        vector<int> r(rowIndex + 1);
        r[0] = 1;
        r[rowIndex] = 1;
        for (int i = 1; i < (rowIndex+2) / 2; i++) {
            r[i] = long(r[i-1]) * (rowIndex - i + 1) / i;
            r[rowIndex - i] = r[i];
        }
        return r;
    }
};

int main() {
    const int k = 33;
    printf("[");
    for (int n : Solution().getRow(k)) {
        printf("%d,", n);
    }
    printf("\b]\n");
    return 0;
}