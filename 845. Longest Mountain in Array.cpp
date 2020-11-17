#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int longestMountain(vector<int>& A) {
        int ac = 0, dc = 0, n = 0;
        for (int i = 1; i < A.size(); i++) {
            if (A[i] > A[i-1]) {
                if (dc > 0) {
                    dc = 0;
                    ac = 0;
                }
                ac++;
            }
            else if (A[i] < A[i-1]) {
                dc++;
                if (ac > 0) {
                    n = max(ac + dc + 1, n);
                }
            }
            else {
                ac = 0;
                dc = 0;
            }
        }
        return n;
    }
};

int main() {
    vector<int> array = {2,1,4,7,3,2,5};
    printf("%d\n", Solution().longestMountain(array));
    return 0;
}