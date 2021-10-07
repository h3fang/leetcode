#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    string getPermutation(int n, int k) {
        vector<int> fs(n, 0);
        string s(n, '1');
        string r(n, 0);
        fs[0] = 1;
        for (int i=1; i<n; i++) {
            fs[i] = fs[i-1] * i;
            s[i] = i + '1';
        }
        for (int i=0; i<n; i++) {
            int d = 0;
            while (fs[n-i-1] < k) {
                k -= fs[n-i-1];
                d++;
            }
            r[i] = s[d];
            s.erase(d, 1);
        }
        return r;
    }
};

int main() {
    printf("%s\n", Solution().getPermutation(4, 4).data());
    return 0;
}