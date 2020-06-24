#include <cstdio>
#include <vector>
#include <cmath>
#include <stack>
#include "helpers.h"

using namespace std;

class Solution {
public:
    int countNodes(TreeNode* root) {
        int hl = 0, hr = 0;
        TreeNode *l = root, *r = root;
        while(l) {
            hl++;
            l = l->left;
        }
        while(r) {
            hr++;
            r = r->right;
        }
        if (hl == hr) { return pow(2, hl) - 1;}
        return 1 + countNodes(root->left) + countNodes(root->right);
    }
};

int main() {
    vector<int> nums = {1,2,3,4,5,6};
    auto root = parse_tree(nums);
    printf("%d\n", Solution().countNodes(root));
    return 0;
}