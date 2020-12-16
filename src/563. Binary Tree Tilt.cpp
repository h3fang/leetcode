#include <cstdio>
#include <vector>
#include <cmath>
#include "helpers.h"

using namespace std;

class Solution {
public:
    pair<int, int> sum(TreeNode* root) {
        if (!root) {
            return {0, 0};
        }
        auto l = sum(root->left);
        auto r = sum(root->right);
        return {l.first + r.first + root->val, abs(l.first - r.first) + l.second + r.second};
    }
    int findTilt(TreeNode* root) {
        if (!root) {
            return 0;
        }
        auto l = sum(root->left);
        auto r = sum(root->right);
        return abs(l.first - r.first) + l.second + r.second;
    }
};

int main() {
    vector<int> nums = {21,7,14,1,1,2,2,3,3};
    auto root = parse_tree(nums);
    auto r = Solution().findTilt(root);
    printf("%d\n", r);
    return 0;
}