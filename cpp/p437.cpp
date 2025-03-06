#include "helpers.h"
#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
    unordered_map<long, int> m;

    int dfs(TreeNode *root, long curr, int sum) {
        if (!root) {
            return 0;
        }
        int ans = 0;
        curr += root->val;
        ans += m[curr - sum];
        m[curr] += 1;
        ans += dfs(root->left, curr, sum) + dfs(root->right, curr, sum);
        m[curr] -= 1;
        return ans;
    }

public:
    int pathSum(TreeNode *root, int sum) {
        m[0] = 1;
        return dfs(root, 0, sum);
    }
};

int main() {
    vector<int> nodes = {10, 5, -3, 3, 2, null, 11, 3, -2, null, 1};
    const int sum = 8;
    auto root = parse_tree(nodes);
    assert(3 == Solution().pathSum(root, sum));
    return 0;
}