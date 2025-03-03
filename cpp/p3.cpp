#include <array>
#include <cassert>
#include <string>
using namespace std;

class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        int ans = 0, l = 0;
        array<int, 128> last;
        last.fill(-1);
        for (int r = 0; r < s.size(); r++) {
            l = max(last[s[r]] + 1, l);
            ans = max(ans, r - l + 1);
            last[s[r]] = r;
        }
        return ans;
    }
};

int main() {
    string s = " ";
    assert(1 == Solution().lengthOfLongestSubstring(s));
    return 0;
}