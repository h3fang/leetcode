#include <algorithm>
#include <vector>
#include <string>
#include <map>
#include <set>
#include <cstdio>

using namespace std;

class Solution {
public:
    vector<string> res;
    void gp(int n, int m, int a, int b, int l, int r, string& s) {
        if (n==0 && m==0) {
            res.push_back(s);
            return;
        }

        if (n>0 && m>0) {
            s[a] = '('; s[b] = ')';
            gp(n-1, m-1, a+1, b-1, l+1, r+1, s);


            if( l > 0 && r > 0) {
                s[a] = ')'; s[b] = '(';
                gp(n-1, m-1, a+1, b-1, l-1, r-1, s);
            }
        }

        if( n > 1 && r > 0) {
            s[a] = '('; s[b] = '(';
            gp(n-2, m, a+1, b-1, l+1, r-1, s);
        }

        if( m > 1 && l > 0) {
            s[a] = ')'; s[b] = ')';
            gp(n, m-2, a+1, b-1, l-1, r+1, s);
        }
    }
    vector<string> generateParenthesis(int n) {
        if (n==0) return vector<string>{""};
        string s(2*n, 0);
        gp(n, n, 0, 2*n-1, 0, 0, s);
        return res;
    }
};

int main() {
    for (auto& e : Solution().generateParenthesis(10)) {
        printf("%s\n", e.data());
    }
    return 0;
}