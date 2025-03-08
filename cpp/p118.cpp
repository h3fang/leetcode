#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> generate(int n) {
        vector<vector<int>> result;
        result.reserve(n);
        result.push_back(vector<int>(1, 1));

        for (int i = 1; i < n; i++) {
            auto &p = result[i - 1];
            vector<int> r(i + 1);
            r[0] = 1;
            for (int j = 1; j < i; j++) {
                r[j] = p[j] + p[j - 1];
            }
            r[i] = 1;
            result.push_back(r);
        }

        return result;
    }
};

int main() {
    auto r = Solution().generate(5);
    vector<vector<int>> expected = {{1}, {1, 1}, {1, 2, 1}, {1, 3, 3, 1}, {1, 4, 6, 4, 1}};
    assert(r == expected);
    return 0;
}