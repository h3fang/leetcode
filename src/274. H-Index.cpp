#include <vector>
#include <cstdio>

using namespace std;

class Solution {
public:
    int hIndex(vector<int>& citations) {
        const int N = citations.size();
        if (N == 0) { return 0; }
        vector<int> m(N+1);
        for (int c : citations) {
            if (c >= N) {
                m[N]++;
            }
            else {
                m[c]++;
            }
        }

        int c = 0;
        for (int i=N; i>=0; i--) {
            c += m[i];
            if (c >= i) {
                return i;
            }
        }

        return 0;
    }
};

int main() {
    vector<int> citations = {3,0,6,1,5};
    printf("%d\n", Solution().hIndex(citations));
    return 0;
}