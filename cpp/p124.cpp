#include "helpers.h"
#include <cassert>
#include <climits>
#include <vector>

using namespace std;

class Solution {
    int ans = INT_MIN;
    int dfs(TreeNode *root) {
        if (!root) {
            return 0;
        }
        int l = dfs(root->left), r = dfs(root->right);
        ans = max(ans, l + r + root->val);
        return max(max(l, r) + root->val, 0);
    }

public:
    int maxPathSum(TreeNode *root) {
        dfs(root);
        return ans;
    }
};

int main() {
    auto root = parse_tree({1, 2, 3});
    assert(6 == Solution().maxPathSum(root));

    root = parse_tree({-10, 9, 20, null, null, 15, 7});
    assert(42 == Solution().maxPathSum(root));
    return 0;
}