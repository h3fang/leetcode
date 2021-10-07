#include <cstdio>
#include <vector>
#include <cmath>
#include "helpers.h"

using namespace std;

class Solution {
    int result = 0;

public:
    pair<int, int> traverse(TreeNode* root) {
        int maximum = root->val, minimum = root->val;
        if (root->left) {
            const auto maxmin = traverse(root->left);
            result = max(max(abs(root->val - maxmin.first), abs(root->val - maxmin.second)), result);
            maximum = max(maxmin.first, maximum);
            minimum = min(maxmin.second, minimum);
        }
        if (root->right) {
            const auto maxmin = traverse(root->right);
            result = max(max(abs(root->val - maxmin.first), abs(root->val - maxmin.second)), result);
            maximum = max(maxmin.first, maximum);
            minimum = min(maxmin.second, minimum);
        }
        return {maximum, minimum};
    }

    int maxAncestorDiff(TreeNode* root) {
        result = 0;
        traverse(root);
        return result;
    }
};

int main() {
    vector<int> nums = {8,3,10,1,6,null,14,null,null,4,7,13};
    auto root = parse_tree(nums);
    auto r = Solution().maxAncestorDiff(root);
    printf("%d\n", r);
    return 0;
}