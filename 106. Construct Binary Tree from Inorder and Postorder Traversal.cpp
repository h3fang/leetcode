#include <cstdio>
#include <unordered_map>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    TreeNode *buildTree(vector<int> &inorder, vector<int> &postorder, int in_start, int in_end, int &post_idx,
                        unordered_map<int, int> &m) {
        if (in_start > in_end) {
            return nullptr;
        }
        auto r = new TreeNode(postorder[post_idx]);
        post_idx--;
        if (in_start == in_end) {
            return r;
        }
        int v = m[r->val];
        r->right = buildTree(inorder, postorder, v + 1, in_end, post_idx, m);
        r->left = buildTree(inorder, postorder, in_start, v - 1, post_idx, m);

        return r;
    }

    TreeNode *buildTree(vector<int> &inorder, vector<int> &postorder) {
        int post_idx = inorder.size() - 1;
        unordered_map<int, int> m;
        for (int i = 0; i <= post_idx; i++) {
            m[inorder[i]] = i;
        }
        return buildTree(inorder, postorder, 0, post_idx, post_idx, m);
    }
};

int main() {
    vector<int> inorder = {9, 3, 15, 20, 7};
    vector<int> postorder = {9, 15, 7, 20, 3};

    auto r = Solution().buildTree(inorder, postorder);
    print_tree(r);
    return 0;
}