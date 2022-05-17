#include "helpers.h"

#include <cassert>

class Solution {
public:
    TreeNode *getTargetCopy(TreeNode *original, TreeNode *cloned, TreeNode *target) {
        if (!original) {
            return nullptr;
        }
        if (original == target) {
            return cloned;
        }
        auto l = getTargetCopy(original->left, cloned->left, target);
        if (l) {
            return l;
        }
        return getTargetCopy(original->right, cloned->right, target);
    }
};

int main() {
    auto original = parse_tree({7, 4, 3, null, null, 6, 19});
    auto cloned = parse_tree({7, 4, 3, null, null, 6, 19});
    auto target = find_node(original, 3);
    auto expected = find_node(cloned, 3);
    auto result = Solution().getTargetCopy(original, cloned, target);
    assert(expected == result);
    return 0;
}