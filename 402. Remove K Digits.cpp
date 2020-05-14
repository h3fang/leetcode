#include <vector>
#include <string>
#include <set>
#include <algorithm>
#include <cstdio>

using namespace std;

class Solution {
public:
    string removeKdigits(string num, int k) {
        const int N = num.size(), d = N-k;
        if (d == 0) {return "0";}
        string s(N, 0);
        int top = 0;
        for (const auto c:num) {
            while (top>0 && s[top-1]>c && k>0) {
                top--;
                k--;
            }
            s[top] = c;
            top++;
        }
        int a = 0;
        while (a<s.size() && s[a] == '0') {
            a++;
        }
        if (a==d) {
            return "0";
        }
        else {
            return s.substr(a, d-a);
        }
    }
};

int main() {
    auto e = Solution().removeKdigits("111111111111111111111111111111111111",10);
    printf("%s\n", e.data());
    return 0;
}