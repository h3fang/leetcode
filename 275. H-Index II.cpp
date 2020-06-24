#include <vector>
#include <cstdio>

using namespace std;

class Solution {
public:
    int hIndex(vector<int>& citations) {
        const int N = citations.size();
        if (N == 0) { return 0;}
        int a = 0, b = N - 1;
        while (a<b) {
            int c = a + (b-a)/2;
            if (citations[c] < N - c) {
                a = c+1;
            } else {
                b = c;
            }
        }
        return N-b;
    }
};

int main() {
    vector<int> citations = {2};//{0,1,3,5,6};//{3,4,5,8,10};
    printf("%d\n", Solution().hIndex(citations));
    return 0;
}