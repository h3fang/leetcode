#include <vector>
#include <queue>
#include <cstdio>

using namespace std;

class Solution {
public:
    int kadane(vector<int>& A) {
        int c = A[0], r = c;
        for (int i=1; i<A.size(); i++) {
            c = max(c+A[i], A[i]);
            if (r < c) r = c;
        }
        return r;
    }

    int maxSubarraySumCircular(vector<int>& A) {
        const int N = A.size();
        int r = kadane(A);
        vector<int> rightsums(N, 0);
        rightsums[N-1] = A[N-1];
        for (int j=N-2; j>=0; j--) {
            rightsums[j] = rightsums[j+1] + A[j];
        }

        vector<int> maxright(N, 0);
        maxright[N-1] = rightsums[N-1];
        for (int j=N-2; j>=0; j--) {
            maxright[j] = max(maxright[j+1], rightsums[j]);
        }

        int leftsum = 0;
        for (int i=0; i<N-2; i++) {
            leftsum += A[i];
            r = max(r, leftsum + maxright[i+2]);
        }

        return r;
    }
};

int main() {
    vector<int> inputs = {-1,3,-3,9,-6,8,-5,-5,-6,10};

    printf("%d\n", Solution().maxSubarraySumCircular(inputs));
    return 0;
}