#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
    TreeNode *dfs(vector<int> &preorder, vector<int> &inorder, const int a, const int b,
                  const int c, const int d) {
        if (a > b) {
            return nullptr;
        }
        int val = preorder[a];
        int x = 0;
        while (inorder[c + x] != val) {
            x += 1;
        }
        auto l = dfs(preorder, inorder, a + 1, a + x, c, c + x - 1);
        auto r = dfs(preorder, inorder, a + x + 1, b, c + x + 1, d);
        return new TreeNode(val, l, r);
    }

public:
    TreeNode *buildTree(vector<int> &preorder, vector<int> &inorder) {
        int n = preorder.size();
        return dfs(preorder, inorder, 0, n - 1, 0, n - 1);
    }
};

int main() {
    vector<int> preorder = {3, 9, 20, 15, 7};
    vector<int> inorder = {9, 3, 15, 20, 7};
    auto r = Solution().buildTree(preorder, inorder);
    auto expected = parse_tree({3, 9, 20, null, null, 15, 7});
    assert(equal(r, expected));
    return 0;
}
