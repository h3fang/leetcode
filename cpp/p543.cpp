#include "helpers.h"
#include <cassert>
#include <vector>

using namespace std;

class Solution {
    int ans;
    int dfs(TreeNode *root) {
        if (!root) {
            return 0;
        }
        int l = dfs(root->left);
        int r = dfs(root->right);
        ans = max(ans, l + r + 1);
        return max(l, r) + 1;
    }

public:
    int diameterOfBinaryTree(TreeNode *root) {
        ans = 0;
        dfs(root);
        return ans - 1;
    }
};

int main() {
    vector<int> nodes = {1, 2, 3, 4, 5};
    auto root = parse_tree(nodes);
    int r = Solution().diameterOfBinaryTree(root);
    assert(r == 3);
    return 0;
}
