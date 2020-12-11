#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> sortArrayByParity(vector<int>& A) {
        int b = A.size() - 1;
        for (int i = 0; i <= b; i++) {
            if (A[i] % 2 == 1) {
                swap(A[i--], A[b--]);
            }
        }
        return A;
    }
};

int main() {
    vector<int> nums = {1,2,3,0,2};
    printf("[");
    for (int n : Solution().sortArrayByParity(nums)) {
        printf("%d,", n);
    }
    printf("\b]\n");
    return 0;
}