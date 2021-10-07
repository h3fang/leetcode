#include <cstdio>
#include <vector>
#include <cmath>
#include "helpers.h"

using namespace std;

class Solution {
public:
    int maxDepth(TreeNode* root) {
        if (!root) {
            return 0;
        }
        return 1 + max(maxDepth(root->left), maxDepth(root->right));
    }
};

int main() {
    vector<int> nodes = {3,9,20,null,null,15,7};
    auto root = parse_tree(nodes);
    printf("%d\n", Solution().maxDepth(root));
    return 0;
}
