#include <cstdio>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    int widthOfBinaryTree(TreeNode *root) {
        if (!root) {
            return 0;
        }
        vector<pair<TreeNode *, long>> level{{root,0}};
        int r = 1;
        while (true) {
            vector<pair<TreeNode *, long>> next_level;
            int a = -1, b = -1;
            for (auto [n, k] : level) {
                if (a < 0) {
                    a = k;
                }
                b = k;
                if (a >= 0) {
                    if (n->left) {
                        next_level.push_back({n->left, 2*k});
                    }
                    if (n->right) {
                        next_level.push_back({n->right, 2*k+1});
                    }
                }
            }
            r = max(b - a + 1, r);
            if (next_level.size() == 0) {
                break;
            }
            level = move(next_level);
        }
        return r;
    }
};

int main() {
    vector<int> nums = {1, 3, 2, 5, 3, NULL_NODE, 9};
    TreeNode *root = parse_tree(nums);
    printf("%d\n", Solution().widthOfBinaryTree(root));
    return 0;
}