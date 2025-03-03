#include <algorithm>
#include <array>
#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        array<int, 26> f = {0};
        for (char c : p) {
            f[c - 'a'] += 1;
        }
        int n = p.size(), l = 0;
        vector<int> ans;
        for (int r = 0; r < s.size(); r++) {
            char c = s[r] - 'a';
            f[c] -= 1;
            while (f[c] < 0) {
                f[s[l] - 'a'] += 1;
                l += 1;
            }
            if (r - l + 1 == n) {
                ans.push_back(l);
            }
        }
        return ans;
    }
};

int main() {
    auto r = Solution().findAnagrams("cbaebabacd", "abc");
    sort(r.begin(), r.end());
    vector<int> expected = {0, 6};
    assert(r == expected);
    return 0;
}