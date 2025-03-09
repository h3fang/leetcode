#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int longestCommonSubsequence(string s, string t) {
        int m = s.size(), n = t.size();
        vector<int> f(n + 1, 0);
        for (int i = 1; i <= m; i++) {
            int pre = 0;
            for (int j = 1; j <= n; j++) {
                int temp = f[j];
                if (s[i - 1] == t[j - 1]) {
                    f[j] = pre + 1;
                } else {
                    f[j] = max(f[j - 1], f[j]);
                }
                pre = temp;
            }
        }
        return f[n];
    }
};

int main() {
    auto r = Solution().longestCommonSubsequence("abcde", "ace");
    assert(3 == r);

    r = Solution().longestCommonSubsequence("abc", "abc");
    assert(3 == r);

    r = Solution().longestCommonSubsequence("abc", "def");
    assert(0 == r);
    return 0;
}