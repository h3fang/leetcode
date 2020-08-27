#include <cstdio>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    int dfs(TreeNode* root, int sum) {
        if (!root) {
            return 0;
        }
        return root->val == sum ? 1 : 0 + dfs(root->left, sum - root->val) + dfs(root->right, sum - root->val);
    }
    int pathSum(TreeNode* root, int sum) {
        if (!root) {
            return 0;
        }
        return dfs(root, sum) + pathSum(root->left, sum) + pathSum(root->right, sum);
    }
};

int main() {
    vector<int> nodes = {10,5,-3,3,2,NULL_NODE,11,3,-2,NULL_NODE,1};
    const int sum = 8;
    auto root =parse_tree(nodes);
    printf("%d\n", Solution().pathSum(root, sum));
    return 0;
}