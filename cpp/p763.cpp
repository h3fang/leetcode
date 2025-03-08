#include <array>
#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> partitionLabels(string s) {
        array<int, 26> last;
        last.fill(-1);
        for (int i = 0; i < s.size(); i++) {
            last[s[i] - 'a'] = i;
        }
        vector<int> ans;
        for (int start = 0, end = 0; start < s.size();) {
            for (int j = start; j <= end; j++) {
                int k = last[s[j] - 'a'];
                end = max(end, k);
            }
            ans.push_back(end - start + 1);
            end += 1;
            start = end;
        }
        return ans;
    }
};

int main() {
    auto r = Solution().partitionLabels("ababcbacadefegdehijhklij");
    vector<int> expected = {9, 7, 8};
    assert(r == expected);
    return 0;
}