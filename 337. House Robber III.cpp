#include <cstdio>
#include <vector>
#include <unordered_map>

#include "helpers.h"

using namespace std;

class Solution {
    unordered_map<TreeNode*, int> dp;
public:
    int rob(TreeNode* root) {
        if (!root) {
            return 0;
        }
        if (dp.find(root) != dp.end()) {
            return dp[root];
        }
        int a = root->val;
        if (root->left) {
            a += rob(root->left->left) + rob(root->left->right);
        }
        if (root->right) {
            a += rob(root->right->left) + rob(root->right->right);
        }

        int b = rob(root->left) + rob(root->right);
        a = max(a,b);
        dp[root] = a;
        return a;
    }
};

int main() {
    vector<int> nodes = {3,2,3,null,3,null,1};
    auto root = parse_tree(nodes);
    printf("%d\n", Solution().rob(root));
    return 0;
}
