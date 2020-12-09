#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int numPairsDivisibleBy60(vector<int>& time) {
        vector<int> count(60);
        for (int t : time) {
            count[t%60]++;
        }
        int r = count[0] * (count[0] - 1) / 2 + count[30] * (count[30] - 1) / 2;
        for (int i = 1; i < 30; i++) {
            r += count[i] * count[60-i];
        }
        return r;
    }
};

int main() {
    vector<int> time = {30,20,150,100,40};
    printf("%d\n", Solution().numPairsDivisibleBy60(time));
    return 0;
}